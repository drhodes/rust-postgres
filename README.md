Bindings for postgres (libpq 9.1.1)
=============

Usage
-----

use pq;
-------

Testing
-------

To test the bindings you'll need to get into postgres
and create a user and a database.  

Here's the session I used to do just this:

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