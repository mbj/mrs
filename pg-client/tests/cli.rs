#[cfg(feature = "clap")]
mod cli_help {
    use clap::{CommandFactory, Parser, Subcommand};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Parser)]
    #[command(name = "pg-client")]
    struct App {
        #[command(subcommand)]
        command: Command,
    }

    #[derive(Debug, Subcommand)]
    enum Command {
        #[command(name = "partitioned-index")]
        PartitionedIndex(pg_client::sqlx::partitioned_index::cli::Cli),
    }

    #[test]
    fn partitioned_index_help() {
        let cmd = App::command();
        let subcommand = cmd
            .find_subcommand("partitioned-index")
            .expect("partitioned-index subcommand")
            .clone();

        let help = subcommand.term_width(80).render_help().to_string();
        let expected = indoc! {r#"
            Add an index to a partitioned PostgreSQL table

            Usage: partitioned-index [OPTIONS] --table <TABLE> --index <INDEX> --key-expression <KEY_EXPRESSION>

            Options:
                  --table <TABLE>
                      Parent partitioned table name
                  --index <INDEX>
                      Index name for the parent index
                  --key-expression <KEY_EXPRESSION>
                      Index key expression without surrounding parentheses (e.g. "created_at" or "lower(email), account_id")
                  --schema <SCHEMA>
                      Schema name [default: public]
                  --unique
                      Create a unique index
                  --method <METHOD>
                      Index access method (e.g. btree, hash) [default: btree]
                  --where-clause <WHERE_CLAUSE>
                      WHERE clause filter (without the WHERE keyword)
                  --concurrently
                      Use CREATE INDEX CONCURRENTLY on partitions
                  --jobs <JOBS>
                      Number of parallel workers for partition index creation [default: 1]
              -h, --help
                      Print help
        "#};

        assert_eq!(help, expected);
    }
}
