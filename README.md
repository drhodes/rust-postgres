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

running 5 tests
NOTICE:  table "movie" does not exist, skipping
NOTICE:  table "movie4" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie4_did_seq" for serial column "movie4.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie_did_seq" for serial column "movie.did"
NOTICE:  table "movie1" does not exist, skipping
NOTICE:  table "movie2" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie1_did_seq" for serial column "movie1.did"
NOTICE:  CREATE TABLE will create implicit sequence "movie2_did_seq" for serial column "movie2.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie4_did_key" for table "movie4"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie2_did_key" for table "movie2"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie1_did_key" for table "movie1"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_did_key" for table "movie"
NOTICE:  table "movie3" does not exist, skipping
NOTICE:  CREATE TABLE will create implicit sequence "movie3_did_seq" for serial column "movie3.did"
test glue::GetAllRowTest ... ok
test ResultTest ... ok
test glue::UseCase2 ... ok
test glue::GetRowTest ... ok
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie3_did_key" for table "movie3"
test glue::UseCase1 ... ok

result: ok. 5 passed; 0 failed; 0 ignored

</pre>