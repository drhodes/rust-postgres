Bindings for postgres (libpq 9.1.1)
=============
These aren't ready yet :)


Testing
-------

To test the bindings you'll need to get into postgres as a superuser and create a user 

Here's what my session looked like

<pre>
~$ psql
psql (9.1.1)
Type "help" for help.

postgres=# <b>CREATE USER rustuser WITH PASSWORD 'rustpass';</b>
<i>CREATE ROLE</i>

postgres=# <b>CREATE DATABASE rust_test_db;</b>
<i>CREATE DATABASE</i>

</pre>


Preliminary Results
-------------------

If postgres is setup correctly and rustc can find your libpq then you might be 
able to get the following output

<pre>

$ rustc --bin pq.rc --test && ./pq

running 2 tests
rust: ~"CONNECTION_OK"
rust: ~"CONNECTION_OK"
rust: 90101
rust: 90101
rust: 90101
rust: 90101
rust: 3
rust: 3
NOTICE:  table "movie1" does not exist, skipping
NOTICE:  table "movie" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie1_did_seq" for serial column "movie1.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie_did_seq" for serial column "movie.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_did_key" for table "movie"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie1_did_key" for table "movie1"
rust: ~"PGRES_COMMAND_OK"
rust: ~"-------------------------------------------------------"
rust: ~[Int32(1), VarChar(~"a new hope"), Int32(1977), VarChar(~"lucas")]
rust: ~"CONNECTION_OK"
test glue::ResultTest ... ok
test ResultTest ... ok

result: ok. 2 passed; 0 failed; 0 ignored

</pre>