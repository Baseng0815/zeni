use std::collections::HashMap;
use std::fmt::{
    Display,
    Formatter,
};
use std::path::Path;
use std::{
    fmt,
    fs,
    io,
};

use petgraph::Graph;
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;

use crate::account::{
    Account,
    AccountId,
};
use crate::ledger::Ledger;
use crate::money::Money;
use crate::transaction::Transaction;

enum Node<'a> {
    Account(&'a Account),
    Transaction(&'a Transaction),
}

impl Ledger {
    pub fn save_as_graph(
        &self,
        path: impl AsRef<Path>,
    ) -> io::Result<()> {
        let graph = self.build_graph()?;
        let dot =
            Dot::with_attr_getters(&graph, &[], &|_, _| String::new(), &|_, (_, node)| node.attributes()).to_string();

        fs::write(path, dot)
    }

    fn build_graph(&self) -> io::Result<Graph<Node<'_>, &Money>> {
        let mut graph = Graph::new();
        let account_nodes: HashMap<AccountId, NodeIndex> = self
            .accounts
            .iter()
            .map(|account| (account.id(), graph.add_node(Node::Account(account))))
            .collect();

        for transaction in &self.transactions {
            let transaction_node = graph.add_node(Node::Transaction(transaction));

            for entry in transaction.entries() {
                let account_node = *account_nodes
                    .get(&entry.account())
                    .ok_or_else(|| unknown_account(entry.account()))?;
                let amount = entry.amount();

                if amount.is_negative() {
                    graph.add_edge(account_node, transaction_node, amount);
                } else {
                    graph.add_edge(transaction_node, account_node, amount);
                }
            }
        }

        Ok(graph)
    }
}

fn unknown_account(account: AccountId) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("transaction entry references unknown account {account}"))
}

impl Node<'_> {
    fn attributes(&self) -> String {
        match self {
            Node::Account(_) => String::new(),
            Node::Transaction(_) => "shape = box".to_owned(),
        }
    }
}

impl Display for Node<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Node::Account(account) => account.fmt(f),
            Node::Transaction(transaction) => {
                write!(f, "{} ({})", transaction.description(), transaction.created_at().strftime("%F"))
            }
        }
    }
}
