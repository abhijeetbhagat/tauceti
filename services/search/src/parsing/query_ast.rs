#[derive(PartialEq, Debug)]
pub enum QueryExpression {
    And(Box<QueryExpression>, Box<QueryExpression>),
    Or(Box<QueryExpression>, Box<QueryExpression>),
    Term(String),
}
