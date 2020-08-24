use crate::parsing::query_ast::QueryExpression;
use crate::trees::index_tree::IndexTree;
use async_std::sync::RwLock;
use futures::future::{BoxFuture, FutureExt};
use std::{collections::HashSet, sync::Arc};

/*
/// Walks over the query expression AST and performs set intersection/union operations
///
/// by retrieving the sets from the index-tree while the walking is done.
fn walk<K, V, Q: ?Sized>(
    ast: &QueryExpression,
    tree: Arc<RwLock<IndexTree<K, V>>>,
) -> BoxFuture<'static, HashSet<V>>
where
    K: Borrow<Q> + Borrow<String> + Eq + Hash + Send + Sync,
    Q: Eq + Hash,
    V: Eq + Hash + Clone + Send + Sync,
{
    async move {
        match ast {
            QueryExpression::And(op1, op2) => {
                let result = walk(op1, tree.clone()).await;
                return result
                    .intersection(&walk(op2, tree.clone()).await)
                    .cloned()
                    .collect::<HashSet<V>>();
            }
            QueryExpression::Or(op1, op2) => {
                let result = walk(op1, tree.clone()).await;
                return result
                    .union(&walk(op2, tree.clone()).await)
                    .cloned()
                    .collect::<HashSet<V>>();
            }
            QueryExpression::Term(term) => {
                let guard: RwLockReadGuard<'_, IndexTree<K, V>> = tree.read().await;
                let hs = guard.get::<String>(term).unwrap();
                return hs.clone();
            }
        }
    }
    .boxed()
}
*/

/// Walks over the query expression AST and performs set intersection/union operations
///
/// by retrieving the sets from the index-tree while the walking is done.
pub fn walk<'a>(
    ast: &'a QueryExpression,
    tree: Arc<RwLock<IndexTree<String, u32>>>,
) -> BoxFuture<'a, HashSet<u32>> {
    async move {
        match ast {
            QueryExpression::And(op1, op2) => {
                let result = walk(op1, tree.clone()).await;
                return result
                    .intersection(&walk(op2, tree.clone()).await)
                    .cloned()
                    .collect::<HashSet<u32>>();
            }
            QueryExpression::Or(op1, op2) => {
                let result = walk(op1, tree.clone()).await;
                return result
                    .union(&walk(op2, tree.clone()).await)
                    .cloned()
                    .collect::<HashSet<u32>>();
            }
            QueryExpression::Term(term) => {
                let guard = tree.read().await;
                let hs = guard.get(term).unwrap();
                return hs.to_owned();
            }
        }
    }
    .boxed()
}
