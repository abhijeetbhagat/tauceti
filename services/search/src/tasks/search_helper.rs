use crate::parsing::query_ast::QueryExpression;
use crate::trees::index_tree::IndexTree;
use async_std::sync::RwLock;
use futures::future::{BoxFuture, FutureExt};
use std::{collections::HashSet, sync::Arc};

/// Walks over the query expression AST and performs set intersection/union operations
///
/// by retrieving the sets from the index-tree while the walking is done.
pub fn walk<'a>(
    ast: &'a QueryExpression,
    tree: &'a IndexTree<String, u32>,
) -> BoxFuture<'a, HashSet<u32>> {
    async move {
        match ast {
            QueryExpression::And(op1, op2) => {
                let result = walk(op1, tree).await;
                return result
                    .intersection(&walk(op2, tree).await)
                    .cloned()
                    .collect::<HashSet<u32>>();
            }
            QueryExpression::Or(op1, op2) => {
                let result = walk(op1, tree).await;
                return result
                    .union(&walk(op2, tree).await)
                    .cloned()
                    .collect::<HashSet<u32>>();
            }
            QueryExpression::Term(term) => {
                let hs = tree.get(term).unwrap();
                return hs.to_owned();
            }
        }
    }
    .boxed()
}
