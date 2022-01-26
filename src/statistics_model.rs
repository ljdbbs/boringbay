use crate::schema::statistics::{self, dsl::*};
use anyhow::anyhow;
use chrono::{NaiveDateTime, NaiveTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{Queryable, SqliteConnection};

#[derive(Queryable, Debug, Clone, Insertable)]
#[diesel(table_name = statistics)]
pub struct Statistics {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub membership_id: i64,
    pub page_view: i64,
    pub referrer: i64,
}

impl Statistics {
    pub fn insert_or_update(
        mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
        stat: &Statistics,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(statistics)
            .values(stat)
            .on_conflict((membership_id, created_at))
            .do_update()
            .set((
                page_view.eq(stat.page_view),
                referrer.eq(stat.referrer),
                updated_at.eq(stat.updated_at),
            ))
            .execute(&mut conn)
    }

    pub fn today(
        mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<Statistics>, anyhow::Error> {
        let res = statistics
            .filter(created_at.eq(NaiveDateTime::new(
                Utc::now().date().naive_utc(),
                NaiveTime::from_hms(0, 0, 0),
            )))
            .load::<Statistics>(&mut conn);
        match res {
            Ok(all) => Ok(all),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }

    pub fn all(
        mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<Statistics>, anyhow::Error> {
        let res = statistics.load::<Statistics>(&mut conn);
        match res {
            Ok(all) => Ok(all),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}