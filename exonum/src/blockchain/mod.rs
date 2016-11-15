#[macro_use]
mod spec;
mod block;
mod view;

use std::borrow::Borrow;
use std::ops::Deref;

use time::Timespec;

use ::crypto::{Hash, hash};
use ::messages::{Precommit, Message};
use ::storage::{StorageValue, Patch, Database, Fork, Error, Map, List};

pub use self::block::Block;
pub use self::view::View;

pub trait Blockchain: Sized + Clone + Send + Sync + 'static
    where Self: Deref<Target = <Self as Blockchain>::Database>
{
    type View: View<<<Self as Blockchain>::Database as Database>::Fork, Transaction=Self::Transaction>;
    type Database: Database;
    type Transaction: Message + StorageValue;

    fn last_hash(&self) -> Result<Option<Hash>, Error> {
        self.view().heights().last()
    }

    fn last_block(&self) -> Result<Option<Block>, Error> {
        let view = self.view();
        Ok(match view.heights().last()? {
            Some(hash) => Some(view.blocks().get(&hash)?.unwrap()),
            None => None,
        })
    }

    fn verify_tx(tx: &Self::Transaction) -> bool;
    fn state_hash(fork: &Self::View) -> Result<Hash, Error>;
    fn execute(fork: &Self::View, tx: &Self::Transaction) -> Result<(), Error>;

    // TODO use Iterator to avoid memory allocations?
    fn create_patch(&self,
                    height: u64,
                    round: u32,
                    time: Timespec,
                    txs: &[(Hash, Self::Transaction)])
                    -> Result<(Hash, Vec<Hash>, Patch), Error> {
        // Get last hash
        let last_hash = self.last_hash()?.unwrap_or_else(|| hash(&[]));
        // Create fork
        let fork = self.view();
        // Save & execute transactions
        let mut tx_hashes = Vec::new();
        for &(hash, ref tx) in txs {
            Self::execute(&fork, tx)?;
            fork.transactions()
                .put(&hash, tx.clone())
                .unwrap();
            fork.block_txs(height)
                .append(hash)
                .unwrap();
            tx_hashes.push(hash);
        }
        // Get tx hash
        let tx_hash = fork.block_txs(height).root_hash()?;
        // Get state hash
        let state_hash = Self::state_hash(&fork)?;
        // Create block
        let block = Block::new(height, round, time, &last_hash, &tx_hash, &state_hash);
        // Eval block hash
        let block_hash = block.hash();
        // Update height
        // TODO: check that height == propose.height
        fork.heights().append(block_hash).is_ok();
        // Save block
        fork.blocks().put(&block_hash, block).is_ok();
        Ok((block_hash, tx_hashes, fork.changes()))
    }

    fn commit<'a, I: Iterator<Item = &'a Precommit>>(&self,
                                                     block_hash: Hash,
                                                     patch: &Patch,
                                                     precommits: I)
                                                     -> Result<(), Error> {
        let patch = {
            let view = self.view();
            view.merge(patch);

            for precommit in precommits {
                view.precommits(&block_hash).append(precommit.clone())?;
            }

            view.changes()
        };

        self.merge(&patch)
    }

    fn view(&self) -> Self::View {
        Self::View::from_fork(self.borrow().fork())
    }

    fn merge(&self, patch: &Patch) -> Result<(), Error> {
        self.deref().merge(patch)
    }
}