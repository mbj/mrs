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
        PartitionedIndex {
            #[command(subcommand)]
            command: pg_client::sqlx::partitioned_index::cli::Command,
        },
    }

    #[test]
    fn partitioned_index_create_help() {
        let cmd = App::command();
        let partitioned_index = cmd
            .find_subcommand("partitioned-index")
            .expect("partitioned-index subcommand")
            .clone();
        let create = partitioned_index
            .find_subcommand("create")
            .expect("create subcommand")
            .clone();

        let help = create.term_width(80).render_help().to_string();
        let expected = indoc! {r#"
            Create an index on a partitioned table

            Usage: create [OPTIONS] --table <TABLE> --index <INDEX> --key-expression <KEY_EXPRESSION>

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
                  --include <INCLUDE>
                      INCLUDE clause for covering indexes without INCLUDE keyword or parentheses (e.g. "col1, col2")
                  --where-clause <WHERE_CLAUSE>
                      WHERE clause filter (without the WHERE keyword)
                  --fillfactor <FILLFACTOR>
                      Storage parameter for fillfactor (1-100)
                  --concurrently
                      Use CREATE INDEX CONCURRENTLY on partitions
                  --jobs <JOBS>
                      Number of parallel workers for partition index creation [default: 1]
                  --dry-run
                      Print SQL statements without executing them
              -h, --help
                      Print help
        "#};

        assert_eq!(help, expected);
    }

    #[test]
    fn partitioned_index_gc_help() {
        let cmd = App::command();
        let partitioned_index = cmd
            .find_subcommand("partitioned-index")
            .expect("partitioned-index subcommand")
            .clone();
        let gc = partitioned_index
            .find_subcommand("gc")
            .expect("gc subcommand")
            .clone();

        let help = gc.term_width(80).render_help().to_string();
        let expected = indoc! {r#"
            Garbage collect incomplete index creation state

            Usage: gc [OPTIONS] --index <INDEX>

            Options:
                  --index <INDEX>    Index name for the parent index
                  --schema <SCHEMA>  Schema name [default: public]
                  --jobs <JOBS>      Number of parallel workers for partition index deletion [default: 1]
                  --dry-run          Print SQL statements without executing them
              -h, --help             Print help
        "#};

        assert_eq!(help, expected);
    }
}
