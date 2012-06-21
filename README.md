Bindings for postgres (libpq 9.1.1)
=============
These aren't ready yet :)


Testing
-------

To test the bindings you'll need to get into postgres
and create a user and a database.  

Here's what my session looked like.

<pre>
~$ psql
psql (9.1.1)
Type "help" for help.

postgres=# <b>create user rustuser;</b>
<i>CREATE ROLE</i>

postgres=# <b>alter user rustuser password 'rustpass';</b>
<i>ALTER ROLE</i>

postgres=# <b>create database rust_test_db with owner rustuser;</b>
<i>CREATE DATABASE</i>

</pre>


Preliminary Results
-------------------

If postgres is setup correctly and rustc can find your libpq then you might be 
able to get the following output

<pre>
running 1 tests
rust: "CONNECTION_OK"
rust: 90101
rust: 90101
rust: 3
rust: "rust_test_db"
rust: "rustuser"
rust: "rustpass"
rust: "localhost"
rust: "5432"
rust: ""
rust: ""
rust: 1
rust: "status:    PGRES_COMMAND_OK"
rust: "error msg: "
NOTICE:  CREATE TABLE will create implicit sequence "movie_did_seq" for serial column "movie.did"
NOTICE:  CREATE TABLE / UNIQUE will create implicit index "movie_did_key" for table "movie"
rust: 1
rust: "status:    PGRES_COMMAND_OK"
rust: "error msg: "
rust: 2
rust: "status:    PGRES_TUPLES_OK"
rust: "error msg: "
rust: 3
rust: 4
rust: 0
rust: "title"
rust: 48686
rust: 2
rust: 0
rust: -1
rust: 259
rust: "CmdStatus: SELECT 3"
rust: 0
rust: "3"
rust: "star wars"
rust: 9
rust: 0
rust: 0
rust: "No error found"
test ResultTest ... ok
</pre>