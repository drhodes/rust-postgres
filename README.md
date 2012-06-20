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

<< log into your postgres user >>

~$ psql
psql (9.1.1)
Type "help" for help.

postgres=# create user rustuser;
<i>CREATE ROLE</i>

postgres=# alter user rustuser password 'rustpass';
ALTER ROLE

postgres=# create database rust_test_db with owner rustuser;
CREATE DATABASE
