//! Equivalence of `pg_ephemeral::time_unit` and `pg_ephemeral::definition::Timeout`
//! with PostgreSQL's time-unit parameter parser.
//!
//! Every case is run through our parser and through a live server, which
//! interprets it as the value of `lock_timeout`: an integer parameter with
//! base unit milliseconds, range `0 .. INT_MAX` and `0` disabling the
//! timeout, the same domain as `statement_timeout`. `lock_timeout` is used
//! because setting it cannot interfere with the queries reading it back.

mod common;

use pg_ephemeral::definition::Timeout;
use pg_ephemeral::time_unit;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Postgres {
    Accepts { milliseconds: i64 },
    Rejects,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Ours {
    Timeout(Timeout),
    Syntax,
    FloatRange,
    TimeoutRange,
}

struct Case {
    input: &'static str,
    reference: &'static str,
    postgres: Postgres,
    ours: Ours,
}

const fn accepts(milliseconds: i64) -> Postgres {
    Postgres::Accepts { milliseconds }
}

const fn after(milliseconds: u64) -> Ours {
    Ours::Timeout(Timeout::After(std::time::Duration::from_millis(
        milliseconds,
    )))
}

const DISABLED: Ours = Ours::Timeout(Timeout::Disabled);

/// Every reference, file path and line number, is to the PostgreSQL source at
/// commit c0bf1d89df29e81c6fdad64e0f7cde10f16322bd on REL_18_STABLE:
/// <https://github.com/postgres/postgres/tree/c0bf1d89df29e81c6fdad64e0f7cde10f16322bd>.
/// Line numbers drift on later commits; look them up at that commit.
/// `guc.c` is `src/backend/utils/misc/guc.c`, `guc_tables.c` is
/// `src/backend/utils/misc/guc_tables.c`, other paths are relative to the
/// repository root.
///
/// The first section imports every distinct literal value PostgreSQL's own
/// tests assign to a parameter with a time unit, found by searching
/// `src/test`, `src/bin`, `src/pl` and `contrib` at that commit for
/// assignments to the parameters declared with `GUC_UNIT_MS`, `GUC_UNIT_S` or
/// `GUC_UNIT_MIN`. Each cites its first occurrence. Read as `lock_timeout`, a
/// value can mean something else than in its test, where the parameter may
/// have another base unit or range. The other sections derive from the
/// parser's source.
const CASES: &[Case] = &[
    // PostgreSQL's test suite.
    Case {
        input: "900us",
        reference: "src/test/regress/sql/guc.sql:62 SET vacuum_cost_delay TO '900us'",
        postgres: accepts(1),
        ours: after(1),
    },
    Case {
        input: "1ms",
        reference: "src/test/subscription/t/004_sync.pl:20 wal_retrieve_retry_interval = 1ms",
        postgres: accepts(1),
        ours: after(1),
    },
    Case {
        input: "10ms",
        reference: "contrib/postgres_fdw/sql/query_cancel.sql:19 SET LOCAL statement_timeout = '10ms'",
        postgres: accepts(10),
        ours: after(10),
    },
    Case {
        input: "50ms",
        reference: "src/test/recovery/t/031_recovery_conflict.pl:29 max_standby_streaming_delay = 50ms",
        postgres: accepts(50),
        ours: after(50),
    },
    Case {
        input: "100ms",
        reference: "src/test/isolation/specs/fk-deadlock.spec:22 SET deadlock_timeout = '100ms'",
        postgres: accepts(100),
        ours: after(100),
    },
    Case {
        input: "500ms",
        reference: "src/test/perl/PostgreSQL/Test/Cluster.pm:690 wal_retrieve_retry_interval = '500ms'",
        postgres: accepts(500),
        ours: after(500),
    },
    Case {
        input: "1s",
        reference: "src/test/modules/brin/t/01_workitems.pl:15 autovacuum_naptime=1s",
        postgres: accepts(1_000),
        ours: after(1_000),
    },
    Case {
        input: "3s",
        reference: "src/test/recovery/t/005_replay_delay.pl:32 recovery_min_apply_delay = '${delay}s' with $delay = 3",
        postgres: accepts(3_000),
        ours: after(3_000),
    },
    Case {
        input: "10s",
        reference: "src/test/isolation/specs/deadlock-parallel.spec:70 SET deadlock_timeout = '10s', \
                    src/test/regress/sql/guc.sql:152 rejected by vacuum_cost_delay's range",
        postgres: accepts(10_000),
        ours: after(10_000),
    },
    Case {
        input: "100s",
        reference: "src/test/isolation/specs/deadlock-hard.spec:24 SET deadlock_timeout = '100s'",
        postgres: accepts(100_000),
        ours: after(100_000),
    },
    Case {
        input: "180 s",
        reference: "src/test/postmaster/t/003_start_stop.pl:40 authentication_timeout = '$authentication_timeout s' \
                    with $timeout_default = 180 (src/test/perl/PostgreSQL/Test/Utils.pm:173)",
        postgres: accepts(180_000),
        ours: after(180_000),
    },
    Case {
        input: "600s",
        reference: "src/test/recovery/t/027_stream_regress.pl:63 max_standby_streaming_delay = 600s",
        postgres: accepts(600_000),
        ours: after(600_000),
    },
    Case {
        input: "1min",
        reference: "src/test/recovery/t/044_invalidate_inactive_slots.pl:54 idle_replication_slot_timeout = 1min",
        postgres: accepts(60_000),
        ours: after(60_000),
    },
    Case {
        input: "30min",
        reference: "src/test/recovery/t/039_end_of_wal.pl:120 checkpoint_timeout = '30min'",
        postgres: accepts(1_800_000),
        ours: after(1_800_000),
    },
    Case {
        input: "1h",
        reference: "src/bin/pg_upgrade/t/003_logical_slots.pl:37 checkpoint_timeout = 1h",
        postgres: accepts(3_600_000),
        ours: after(3_600_000),
    },
    Case {
        input: "2h",
        reference: "src/test/regress/sql/psql_pipeline.sql:383 SET LOCAL statement_timeout='2h'",
        postgres: accepts(7_200_000),
        ours: after(7_200_000),
    },
    Case {
        input: "1d",
        reference: "contrib/test_decoding/logical.conf:4 autovacuum_naptime = 1d",
        postgres: accepts(86_400_000),
        ours: after(86_400_000),
    },
    Case {
        input: "0",
        reference: "contrib/auto_explain/t/001_auto_explain.pl:34 auto_explain.log_min_duration = 0",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "1",
        reference: "src/test/modules/test_misc/t/006_signal_autovacuum.pl:26 autovacuum_naptime = 1",
        postgres: accepts(1),
        ours: after(1),
    },
    Case {
        input: "10",
        reference: "src/test/isolation/specs/prepared-transactions-cic.spec:25 SET lock_timeout = 10",
        postgres: accepts(10),
        ours: after(10),
    },
    Case {
        input: "13",
        reference: "src/test/regress/sql/guc.sql:192 SET vacuum_cost_delay = 13",
        postgres: accepts(13),
        ours: after(13),
    },
    Case {
        input: "30",
        reference: "src/test/regress/sql/guc.sql:91 SET LOCAL vacuum_cost_delay TO 30",
        postgres: accepts(30),
        ours: after(30),
    },
    Case {
        input: "40",
        reference: "src/test/regress/sql/guc.sql:9 SET vacuum_cost_delay TO 40",
        postgres: accepts(40),
        ours: after(40),
    },
    Case {
        input: "50",
        reference: "src/test/regress/sql/guc.sql:16 SET LOCAL vacuum_cost_delay TO 50",
        postgres: accepts(50),
        ours: after(50),
    },
    Case {
        input: "60",
        reference: "src/test/regress/sql/guc.sql:36 SET vacuum_cost_delay TO 60",
        postgres: accepts(60),
        ours: after(60),
    },
    Case {
        input: "70",
        reference: "src/test/regress/sql/guc.sql:48 SET vacuum_cost_delay TO 70",
        postgres: accepts(70),
        ours: after(70),
    },
    Case {
        input: "80.1",
        reference: "src/test/regress/sql/guc.sql:53 SET vacuum_cost_delay TO 80.1",
        postgres: accepts(80),
        ours: after(80),
    },
    Case {
        input: "100",
        reference: "src/test/regress/sql/guc.sql:67 SET vacuum_cost_delay TO 100",
        postgres: accepts(100),
        ours: after(100),
    },
    Case {
        input: "600",
        reference: "src/test/postmaster/t/003_start_stop.pl:25 $authentication_timeout = 600",
        postgres: accepts(600),
        ours: after(600),
    },
    Case {
        input: "1000",
        reference: "src/pl/plpgsql/src/sql/plpgsql_trap.sql:102 set statement_timeout to 1000",
        postgres: accepts(1_000),
        ours: after(1_000),
    },
    Case {
        input: "-1",
        reference: "src/bin/pgbench/t/001_pgbench_with_server.pl:368 log_min_duration_statement = -1, \
                    below lock_timeout's minimum of 0 (guc_tables.c:2756)",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    // Units.
    Case {
        input: "2min",
        reference: "guc.c:162 unit min",
        postgres: accepts(120_000),
        ours: after(120_000),
    },
    Case {
        input: "5000us",
        reference: "guc.c:165 unit us",
        postgres: accepts(5),
        ours: after(5),
    },
    Case {
        input: "1500000us",
        reference: "guc.c:165 unit us",
        postgres: accepts(1_500),
        ours: after(1_500),
    },
    Case {
        input: "30S",
        reference: "guc.c:2700 units are case-sensitive",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "30m",
        reference: "guc.c:2700 m is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "1w",
        reference: "guc.c:2700 w is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "30kB",
        reference: "guc.c:2700 memory units do not apply to time parameters",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "30sec",
        reference: "guc.c:2700 sec is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "30mins",
        reference: "guc.c:2689 unit longer than MAX_UNIT_LEN",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "1m30s",
        reference: "guc.c:2689 no compound values",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "30 ms x",
        reference: "guc.c:2689 garbage after the unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    // Whitespace.
    Case {
        input: "30 s",
        reference: "guc.c:2909 whitespace between number and unit",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: "30\tms ",
        reference: "guc.c:2909 whitespace between number and unit, guc.c:2685 after the unit",
        postgres: accepts(30),
        ours: after(30),
    },
    Case {
        input: "30s ",
        reference: "guc.c:2685 whitespace after unit",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: " 30s",
        reference: "guc.c:2894 strtol skips leading whitespace",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: "\n30s\u{b}",
        reference: "guc.c:2894 strtol skips leading whitespace, guc.c:2685 C isspace",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    // Signs.
    Case {
        input: "+30s",
        reference: "guc.c:2894 strtol accepts a sign",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: "-0s",
        reference: "guc.c:2894 strtol accepts a sign",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "-1s",
        reference: "guc.c:3182 below the minimum, guc_tables.c:2756 minimum 0",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "-1ms",
        reference: "guc.c:3182 below the minimum, guc_tables.c:2756 minimum 0",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "-0.4ms",
        reference: "guc.c:2936 rint rounds to -0, which is 0",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "-.5s",
        reference: "guc.c:2895 strtol reads no number at the sign, so no float re-parse",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    // Octal and hexadecimal.
    Case {
        input: "010s",
        reference: "guc.c:2894 strtol base 0 reads a leading zero as octal",
        postgres: accepts(8_000),
        ours: after(8_000),
    },
    Case {
        input: "00s",
        reference: "guc.c:2894 strtol base 0 reads a leading zero as octal",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "08s",
        reference: "guc.c:2894 strtol octal stops at 8, guc.c:2700 8s is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "019s",
        reference: "guc.c:2894 strtol octal stops at 9, guc.c:2700 9s is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "017.5s",
        reference: "guc.c:2895 a decimal point re-parses the octal digits as decimal",
        postgres: accepts(17_500),
        ours: after(17_500),
    },
    Case {
        input: "0x1es",
        reference: "guc.c:2894 strtol base 0 reads 0x as hexadecimal",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: "0X1Es",
        reference: "guc.c:2894 strtol base 0 reads 0X as hexadecimal",
        postgres: accepts(30_000),
        ours: after(30_000),
    },
    Case {
        input: "0xs",
        reference: "guc.c:2894 strtol reads 0 without hexadecimal digits, guc.c:2700 xs is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "0x10.8s",
        reference: "guc.c:2899 strtod reads a hexadecimal fraction",
        postgres: accepts(16_500),
        ours: after(16_500),
    },
    Case {
        input: "0x1.8p1s",
        reference: "guc.c:2899 strtod reads a hexadecimal fraction with binary exponent",
        postgres: accepts(3_000),
        ours: after(3_000),
    },
    Case {
        input: "0x1p3s",
        reference: "guc.c:2895 no float re-parse at p, guc.c:2689 p3s is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    // Fractions and exponents.
    Case {
        input: "1.5s",
        reference: "guc.c:2895 a decimal point re-parses as float",
        postgres: accepts(1_500),
        ours: after(1_500),
    },
    Case {
        input: ".5s",
        reference: "guc.c:2895 a leading decimal point re-parses as float",
        postgres: accepts(500),
        ours: after(500),
    },
    Case {
        input: "1.5min",
        reference: "guc.c:2711 a fraction rounds to the next smaller unit",
        postgres: accepts(90_000),
        ours: after(90_000),
    },
    Case {
        input: "1.5ms",
        reference: "guc.c:2711 rounds to microseconds, guc.c:2936 rint rounds half to even",
        postgres: accepts(2),
        ours: after(2),
    },
    Case {
        input: "0.0005s",
        reference: "guc.c:2711 rounds half to even to 0 milliseconds",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "1e3ms",
        reference: "guc.c:2895 an exponent re-parses as float",
        postgres: accepts(1_000),
        ours: after(1_000),
    },
    Case {
        input: "1E3ms",
        reference: "guc.c:2895 an exponent re-parses as float",
        postgres: accepts(1_000),
        ours: after(1_000),
    },
    Case {
        input: "1.e3ms",
        reference: "guc.c:2899 strtod reads an empty fraction",
        postgres: accepts(1_000),
        ours: after(1_000),
    },
    Case {
        input: "1ems",
        reference: "guc.c:2899 strtod stops at e without exponent digits, guc.c:2700 ems is not a unit",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "e3ms",
        reference: "guc.c:2902 strtod reads no number",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "1e999s",
        reference: "guc.c:2902 strtod overflows",
        postgres: Postgres::Rejects,
        ours: Ours::FloatRange,
    },
    Case {
        input: "1e-400ms",
        reference: "guc.c:2902 strtod underflows",
        postgres: Postgres::Rejects,
        ours: Ours::FloatRange,
    },
    Case {
        input: "1e-310ms",
        reference: "guc.c:2902 strtod underflows to a subnormal",
        postgres: Postgres::Rejects,
        ours: Ours::FloatRange,
    },
    // Not numbers.
    Case {
        input: "",
        reference: "guc.c:2902 no number",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "s",
        reference: "guc.c:2902 no number",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "PT30S",
        reference: "guc.c:2902 no number, ISO 8601 is not accepted",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "NaN",
        reference: "guc.c:2902 no number, src/test/regress/sql/guc.sql:151 SET seq_page_cost TO 'NaN' (a real parameter)",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    Case {
        input: "inf",
        reference: "guc.c:2902 no number, strtol does not read infinity",
        postgres: Postgres::Rejects,
        ours: Ours::Syntax,
    },
    // Zero and rounding.
    Case {
        input: "0ms",
        reference: "guc_tables.c:2752 0 disables the timeout",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "0d",
        reference: "guc_tables.c:2752 0 disables the timeout",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "1us",
        reference: "guc.c:2936 rint rounds to 0, guc_tables.c:2752 0 disables the timeout",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "499us",
        reference: "guc.c:2936 rint rounds down",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "500us",
        reference: "guc.c:2936 rint rounds half to even",
        postgres: accepts(0),
        ours: DISABLED,
    },
    Case {
        input: "501us",
        reference: "guc.c:2936 rint rounds up",
        postgres: accepts(1),
        ours: after(1),
    },
    Case {
        input: "1500us",
        reference: "guc.c:2936 rint rounds half to even",
        postgres: accepts(2),
        ours: after(2),
    },
    Case {
        input: "2500us",
        reference: "guc.c:2936 rint rounds half to even",
        postgres: accepts(2),
        ours: after(2),
    },
    Case {
        input: "3500us",
        reference: "guc.c:2936 rint rounds half to even",
        postgres: accepts(4),
        ours: after(4),
    },
    // Range.
    Case {
        input: "24d",
        reference: "guc.c:160 unit d, largest whole day within INT_MAX ms",
        postgres: accepts(2_073_600_000),
        ours: after(2_073_600_000),
    },
    Case {
        input: "2147483647ms",
        reference: "guc.c:2938 INT_MAX is the largest value",
        postgres: accepts(2_147_483_647),
        ours: after(2_147_483_647),
    },
    Case {
        input: "2147483647.4ms",
        reference: "guc.c:2936 rounds to INT_MAX before the guc.c:2938 range check",
        postgres: accepts(2_147_483_647),
        ours: after(2_147_483_647),
    },
    Case {
        input: "2147483647001us",
        reference: "guc.c:2936 rounds to INT_MAX before the guc.c:2938 range check",
        postgres: accepts(2_147_483_647),
        ours: after(2_147_483_647),
    },
    Case {
        input: "2147483647500us",
        reference: "guc.c:2936 rint rounds half to even past INT_MAX, guc.c:2938 exceeds INT_MAX",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "2147483648ms",
        reference: "guc.c:2938 exceeds INT_MAX",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "25d",
        reference: "guc.c:2938 exceeds INT_MAX",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "-9223372036854775808ms",
        reference: "guc.c:2894 strtol reads LONG_MIN, guc.c:2938 below INT_MIN",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
    Case {
        input: "9223372036854775808ms",
        reference: "guc.c:2895 strtol overflow re-parses as float, guc.c:2938 exceeds INT_MAX",
        postgres: Postgres::Rejects,
        ours: Ours::TimeoutRange,
    },
];

fn parse(input: &str) -> Ours {
    match input.parse::<time_unit::Value>() {
        Ok(value) => match Timeout::try_from(value) {
            Ok(timeout) => Ours::Timeout(timeout),
            Err(_) => Ours::TimeoutRange,
        },
        Err(time_unit::Error::Syntax { .. }) => Ours::Syntax,
        Err(time_unit::Error::Range { .. }) => Ours::FloatRange,
    }
}

#[test]
fn ours_matches_expectations() {
    let expected: Vec<(&str, Ours)> = CASES.iter().map(|case| (case.input, case.ours)).collect();

    let actual: Vec<(&str, Ours)> = CASES
        .iter()
        .map(|case| (case.input, parse(case.input)))
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn ours_is_equivalent_to_postgres() {
    for case in CASES {
        let postgres = match case.ours {
            Ours::Timeout(Timeout::Disabled) => Postgres::Accepts { milliseconds: 0 },
            Ours::Timeout(Timeout::After(duration)) => Postgres::Accepts {
                milliseconds: i64::try_from(duration.as_millis()).unwrap(),
            },
            Ours::Syntax | Ours::FloatRange | Ours::TimeoutRange => Postgres::Rejects,
        };

        assert_eq!(
            postgres, case.postgres,
            "{:?} ({})",
            case.input, case.reference
        );
    }
}

#[tokio::test]
async fn postgres_matches_expectations() {
    let backend = ociman::test_backend_setup!();

    let expected: Vec<(&str, Postgres)> = CASES
        .iter()
        .map(|case| (case.input, case.postgres))
        .collect();

    let actual = common::test_definition(backend, "time-unit".parse().unwrap())
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let mut actual = Vec::new();

                    for case in CASES {
                        actual.push((case.input, postgres(connection, case.input).await));
                    }

                    actual
                })
                .await
        })
        .await
        .unwrap();

    assert_eq!(expected, actual);
}

async fn postgres(connection: &mut sqlx::postgres::PgConnection, input: &str) -> Postgres {
    let result = sqlx::query("SELECT set_config('lock_timeout', $1, false)")
        .bind(input)
        .execute(&mut *connection)
        .await;

    match result {
        Ok(_) => {
            let row = sqlx::query("SELECT setting FROM pg_settings WHERE name = 'lock_timeout'")
                .fetch_one(&mut *connection)
                .await
                .unwrap();

            let setting: String = sqlx::Row::get(&row, 0);

            Postgres::Accepts {
                milliseconds: setting.parse().unwrap(),
            }
        }
        Err(sqlx::Error::Database(error)) => {
            assert_eq!(Some("22023"), error.code().as_deref(), "{input}");
            Postgres::Rejects
        }
        Err(error) => panic!("{input}: {error}"),
    }
}
