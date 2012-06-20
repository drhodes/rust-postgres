// Copyright 2012 Derek A. Rhodes.  All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std;
import io;
import libc::*;
//import result::{ok, err, is_success};
import str::unsafe;

/*-------------------------------------------------------------------------
*
* libpq-fe.h
*     This file contains definitions for structures and
*     externs for functions used by frontend postgres applications.
*
* Portions Copyright (c) 1996-2011, PostgreSQL Global Development Group
* Portions Copyright (c) 1994, Regents of the University of California
*
* src/interfaces/libpq/libpq-fe.h
*
*-------------------------------------------------------------------------
*/


// rhodesd> not sure what to do with these flags... const?

/*
* Option flags for PQcopyResult
*/

// #define PG_COPYRES_ATTRS           0x01;
// #define PG_COPYRES_TUPLES          0x02	/* Implies PG_COPYRES_ATTRS */
// #define PG_COPYRES_EVENTS          0x04
// #define PG_COPYRES_NOTICEHOOKS     0x08


// --------------------------------------------------------------------------
// Application-visible enum types */

enum ConnStatusType {
    CONNECTION_OK,
    CONNECTION_BAD,

    // The existence of these should never be relied upon - they should only
    // be used for user feedback or similar purposes.

    CONNECTION_STARTED,			/* Waiting for connection to be made.  */
    CONNECTION_MADE,			/* Connection OK; waiting to send.     */
    CONNECTION_AWAITING_RESPONSE,		/* Waiting for a response from the
    * postmaster.         */
    CONNECTION_AUTH_OK,			/* Received authentication; waiting for
    * backend startup. */
    CONNECTION_SETENV,			/* Negotiating environment. */
    CONNECTION_SSL_STARTUP,		/* Negotiating SSL. */
    CONNECTION_NEEDED,			/* Internal state: connect() needed */
}

enum PostgresPollingStatusType {
    PGRES_POLLING_FAILED = 0,
    PGRES_POLLING_READING,		/* These two indicate that one may    */
    PGRES_POLLING_WRITING,		/* use select before polling again.   */
    PGRES_POLLING_OK,
    PGRES_POLLING_ACTIVE		/* unused; keep for awhile for backwards
    * compatibility */
}

enum ExecStatusType {
    PGRES_EMPTY_QUERY = 0,		/* empty query string was executed */
    PGRES_COMMAND_OK,			/* a query command that doesn't return
    * anything was executed properly by the
    * backend */
    PGRES_TUPLES_OK,			/* a query command that returns tuples was
    * executed properly by the backend, PGresult
    * contains the result tuples */
    PGRES_COPY_OUT,				// Copy Out data transfer in progress
    PGRES_COPY_IN,				// Copy In data transfer in progress
    PGRES_BAD_RESPONSE,			// an unexpected response was recv'd from the backend
    PGRES_NONFATAL_ERROR,		// notice or warning message
    PGRES_FATAL_ERROR,			// query failed
    PGRES_COPY_BOTH				// Copy In/Out data transfer in progress
}

enum PGTransactionStatusType {
    PQTRANS_IDLE,				/* connection idle */
    PQTRANS_ACTIVE,				/* command in progress */
    PQTRANS_INTRANS,			/* idle, within transaction block */
    PQTRANS_INERROR,			/* idle, within failed transaction */
    PQTRANS_UNKNOWN				/* cannot determine status */
}

enum PGVerbosity {
    PQERRORS_TERSE,				/* singleline error messages */
    PQERRORS_DEFAULT,			/* recommended style */
    PQERRORS_VERBOSE			/* all the facts, ma'am */
}

enum PGPing {
    PQPING_OK,					/* server is accepting connections */
    PQPING_REJECT,				/* server is alive but rejecting connections */
    PQPING_NO_RESPONSE,			/* could not establish connection */
    PQPING_NO_ATTEMPT			/* connection not attempted (bad params) */
}

// /* PGconn encapsulates a connection to the backend.
// * The contents of this struct are not supposed to be known to applications.
// */
// typedef struct pg_conn PGconn;
enum PGconn {}

// /* PGresult encapsulates the result of a query (or more precisely, of a single
// * SQL command --- a query string given to PQsendQuery can contain multiple
// * commands and thus return multiple PGresult objects).
// * The contents of this struct are not supposed to be known to applications.
// */
// typedef struct pg_result PGresult;
enum PGresult {}

// /* PGcancel encapsulates the information needed to cancel a running
// * query on an existing connection.
// * The contents of this struct are not supposed to be known to applications.
// */
// typedef struct pg_cancel PGcancel;
enum PGcancel {}

// PGnotify represents the occurrence of a NOTIFY message.
// Ideally this would be an opaque typedef, but it's so simple that it's
// unlikely to change.

// typedef struct pgNotify
// {
//  char       *relname;		/* notification condition name */
//  int			be_pid;			/* process ID of notifying server process */
//  char       *extra;			/* notification parameter */
//  /* Fields below here are private to libpq; apps should not use 'em */
//  struct pgNotify *next;		/* list link */
// } PGnotify;

enum PGnotify {}  // rhodesd> this might have to be built into a full declaration.


// /* Function types for notice-handling callbacks */
// typedef void (*PQnoticeReceiver) (void *arg, const PGresult *res);
// typedef void (*PQnoticeProcessor) (void *arg, const char *message);

// /* Print options for PQprint() */
// typedef char pqbool;

// typedef struct _PQprintOpt
// {
//  pqbool		header;			/* print output field headings and row count */
//  pqbool		align;			/* fill align the fields */
//  pqbool		standard;		/* old brain dead format */
//  pqbool		html3;			/* output html tables */
//  pqbool		expanded;		/* expand tables */
//  pqbool		pager;			/* use pager for output if needed */
//  char       *fieldSep;		/* field separator */
//  char       *tableOpt;		/* insert to HTML <table ...> */
//  char       *caption;		/* HTML <caption> */
//  char      **fieldName;		/* null terminated array of replacement field
//  * names */
// } PQprintOpt;

// /* ----------------
// * Structure for the conninfo parameter definitions returned by PQconndefaults
// * or PQconninfoParse.
// *
// * All fields except "val" point at static strings which must not be altered.
// * "val" is either NULL or a malloc'd current-value string.  PQconninfoFree()
// * will release both the val strings and the PQconninfoOption array itself.
// * ----------------
// */
// typedef struct _PQconninfoOption
// {
//  char       *keyword;		/* The keyword of the option			*/
//  char       *envvar;			/* Fallback environment variable name	*/
//  char       *compiled;		/* Fallback compiled in default value	*/
//  char       *val;			/* Option's current value, or NULL       */
//  char       *label;			/* Label for field in connect dialog	*/
//  char       *dispchar;		/* Indicates how to display this field in a
//  * connect dialog. Values are: "" Display
//  * entered value as is "*" Password field -
//  * hide value "D"  Debug option - don't show
//  * by default */
//  int			dispsize;		/* Field size in characters for dialog	*/
// } PQconninfoOption;
enum PQconninfoOption {}

// /* ----------------
// * PQArgBlock -- structure for PQfn() arguments
// * ----------------
// */
// typedef struct
// {
//  int			len;
//  int			isint;
//  union
//  {
//      int        *ptr;		/* can't use void (dec compiler barfs)   */
//      int			integer;
//  }			u;
// } PQArgBlock;

// /* ----------------
// * PGresAttDesc -- Data about a single attribute (column) of a query result
// * ----------------
// */
// typedef struct pgresAttDesc
// {
//  char       *name;			/* column name */
//  Oid			tableid;		/* source table, if known */
//  int			columnid;		/* source column, if known */
//  int			format;			/* format code for value (text/binary) */
//  Oid			typid;			/* type id */
//  int			typlen;			/* type size */
//  int			atttypmod;		/* type-specific modifier info */
// } PGresAttDesc;
enum PGresAttDesc {}

enum PQnoticeReceiver {} // assume that just typing this will work. no idea.
enum PQnoticeProcessor {} // assume ""


// /*
// *       Used to set callback that prevents concurrent access to
// *       non-thread safe functions that libpq needs.
// *       The default implementation uses a libpq internal mutex.
// *       Only required for multithreaded apps that use kerberos
// *       both within their app and for postgresql connections.
// */
// typedef void (*pgthreadlock_t) (int acquire);

// extern pgthreadlock_t PQregisterThreadLock(pgthreadlock_t newhandler);

// /* === in fe-exec.c === */



// /*
// * "Fast path" interface --- not really recommended for application
// * use
// */
// extern PGresult *PQfn(PGconn *conn,
//                    int fnid,
//                    int *result_buf,
//                    int *result_len,
//                    int result_is_int,
//                    const PQArgBlock *args,
//                    int nargs);
enum Oid {}




// /* These forms are deprecated! */
// extern size_t PQescapeString(char *to, const char *from, size_t length);
// extern unsigned char *PQescapeBytea(const unsigned char *from, size_t from_length,
//                                  size_t *to_length);



// /* === in fe-print.c === */

// extern void
//     PQprint(FILE *fout,				/* output stream */
//          const PGresult *res,
//          const PQprintOpt *ps);	/* option structure */

// /*
// * really old printing routines
// */
// extern void
//     PQdisplayTuples(const PGresult *res,
//                  FILE *fp,		/* where to send the output */
//                  int fillAlign,	/* pad the fields with spaces */
//                  const char *fieldSep,	/* field separator */
//                  int printHeader,	/* display headers? */
//                  int quiet);

// extern void
//     PQprintTuples(const PGresult *res,
//                FILE *fout,		/* output stream */
//                int printAttName, /* print attribute names */
//                int terseOutput,	/* delimiter bars */
//                int width);		/* width of column, if 0, use variable width */



// #ifdef __cplusplus
// }
// #endif
//     #endif   /* LIBPQ_FE_H */


native mod pq {
    /* ----------------
    * Exported functions of libpq
    * ----------------
    */

    /* ===	in fe-connect.c === */

    // ---------------------------------------------------------------------
    // make a new client connection to the backend
    // Asynchronous (non-blocking)

    // extern PGconn *PQconnectStart(const char *conninfo);
    fn PQconnectStart(conninfo: *c_char) -> *PGconn;

    // extern PGconn *PQconnectStartParams(const char **keywords,
    //                                  const char **values, int expand_dbname);
    fn PQconnectStartParams(keywords: **c_char,
                            values: **c_char, 
                            expand_dbnameL: c_int ) -> *PGconn;

    // extern PostgresPollingStatusType PQconnectPoll(PGconn *conn);
    fn PQconnectPoll(conn: *PGconn) -> PostgresPollingStatusType;

    // ---------------------------------------------------------------------
    // /* Synchronous (blocking) */
    fn PQconnectdb(conninfo: *c_char) -> *PGconn;

    // extern PGconn *PQconnectdbParams(const char **keywords,
    //                               const char **values, int expand_dbname)
    fn PQconnectdbParams(keywords: **char,
                         values: **char, 
                          expand_dbname: c_int) -> *PGconn;

    // extern PGconn *PQsetdbLogin(const char *pghost, const char *pgport,
    //                          const char *pgoptions, const char *pgtty,
    //                          const char *dbName,
    //                          const char *login, const char *pwd);
    fn PQsetdbLogin(pghost: *c_char, 
                    pgport: *c_char,
                    pgoptions: *c_char, 
                    pgtty: *c_char,
                    dbName: *c_char,
                    login: *c_char, 
                    pwd: *c_char) -> *PGconn;

    // #define PQsetdb(M_PGHOST,M_PGPORT,M_PGOPT,M_PGTTY,M_DBNAME)  \
    //     PQsetdbLogin(M_PGHOST, M_PGPORT, M_PGOPT, M_PGTTY, M_DBNAME, NULL, NULL)

    // /* close the current connection and free the PGconn data structure */
    // extern void PQfinish(PGconn *conn);
    fn PQfinish(conn: *PGconn) -> (); //assume -> ()  is the right things to do.

    // /* get info about connection options known to PQconnectdb */
    // extern PQconninfoOption *PQconndefaults(void);
    fn PQconndefaults() -> *PQconninfoOption;

    // /* parse connection options in same way as PQconnectdb */
    // extern PQconninfoOption *PQconninfoParse(const char *conninfo, char **errmsg);
    fn PQconninfoParse(conninfo: *char, errmsg: **char) -> *PQconninfoOption;

    // /* free the data structure returned by PQconndefaults() or PQconninfoParse() */
    // extern void PQconninfoFree(PQconninfoOption *connOptions);
    fn PQconninfoFree(connOptions: *PQconninfoOption) -> (); //rhodesd assumes this is right

    // /*
    // * close the current connection and restablish a new one with the same
    // * parameters
    // */
    // ---------------------------------------------------------------------
    // Asynchronous (non-blocking)

    // extern int	PQresetStart(PGconn *conn);
    fn PQresetStart(conn: *PGconn) -> c_int;

    // extern PostgresPollingStatusType PQresetPoll(PGconn *conn);
    fn PQresetPoll(conn: *PGconn) -> PostgresPollingStatusType;

    // ---------------------------------------------------------------------
    // Synchronous (blocking)

    // extern void PQreset(PGconn *conn);
    fn PQreset(conn: *PGconn) -> (); // assume () is the same as void

    // /* request a cancel structure */
    // extern PGcancel *PQgetCancel(PGconn *conn);
    fn PQgetCancel(conn: *PGconn) -> *PGcancel;

    // /* free a cancel structure */
    // extern void PQfreeCancel(PGcancel *cancel);
    fn PQfreeCancel(cancel: *PGcancel) -> (); // assume () is the same as void

    // /* issue a cancel request */
    // extern int	PQcancel(PGcancel *cancel, char *errbuf, int errbufsize);
    fn PQcancel(cancel: *PGcancel, errbuf: *c_char, errbufsize: c_int) -> c_int;

    // /* backwards compatible version of PQcancel; not thread-safe */
    // extern int	PQrequestCancel(PGconn *conn);
    fn PQrequestCancel(conn: *PGconn) -> c_int;

    // ----------------------------------------------------------------------
    // Accessor functions for PGconn objects

    // extern char *PQdb(const PGconn *conn);
    fn PQdb(conn: *PGconn) -> *c_char;

    // extern char *PQuser(const PGconn *conn);
    fn PQuser(conn: *PGconn) -> *c_char;

    // extern char *PQpass(const PGconn *conn);
    fn PQpass(conn: *PGconn) -> *c_char;

    // extern char *PQhost(const PGconn *conn);
    fn PQhost(conn: *PGconn) -> *c_char;

    // extern char *PQport(const PGconn *conn);
    fn PQport(conn: *PGconn) -> *c_char;

    // extern char *PQtty(const PGconn *conn);
    fn PQtty(conn: *PGconn) -> *c_char;

    // extern char *PQoptions(const PGconn *conn);
    fn PQoptions(conn: *PGconn) -> *c_char;

    // extern ConnStatusType PQstatus(const PGconn *conn);
    fn PQstatus(conn: *PGconn) -> ConnStatusType;

    // extern PGTransactionStatusType PQtransactionStatus(const PGconn *conn);
    fn PQtransactionStatus(conn: *PGconn) -> PGTransactionStatusType;

    // extern const char *PQparameterStatus(const PGconn *conn,
    //                                   const char *paramName);
    fn  PQparameterStatus(conn: *PGconn,
                          paramName: *c_char) -> *c_char;

    // extern int	PQprotocolVersion(const PGconn *conn);
    fn PQprotocolVersion(conn: *PGconn) -> c_int;
    fn PQserverVersion(conn: *PGconn) -> c_int;
    fn PQerrorMessage(conn: *PGconn) -> *c_char;

    // extern int	PQsocket(const PGconn *conn);
    fn PQsocket(conn: *PGconn) -> c_int;

    // extern int	PQbackendPID(const PGconn *conn);
    fn PQbackendPID(conn: *PGconn) -> c_int;

    // extern int	PQconnectionNeedsPassword(const PGconn *conn);
    fn PQconnectionNeedsPassword(conn: *PGconn) -> c_int;

    // extern int	PQconnectionUsedPassword(const PGconn *conn);
    fn PQconnectionUsedPassword(conn: *PGconn) -> c_int;

    // extern int	PQclientEncoding(const PGconn *conn);
    fn PQclientEncoding(conn: *PGconn) -> c_int;

    // extern int	PQsetClientEncoding(PGconn *conn, const char *encoding);
    fn PQsetClientEncoding(conn: *PGconn, encoding: *c_char) -> c_int;


    // ----------------------------------------------------------------------
    // Accessor functions for PGresult objects

    // extern ExecStatusType PQresultStatus(const PGresult *res);
    fn PQresultStatus(res: *PGresult) ->  ExecStatusType;

    // extern char *PQresStatus(ExecStatusType status);
    fn PQresStatus(status: ExecStatusType) -> *c_char;

    // extern char *PQresultErrorMessage(const PGresult *res);
    fn PQresultErrorMessage(res: *PGresult) -> *c_char;

    // extern char *PQresultErrorField(const PGresult *res, int fieldcode);
    fn PQresultErrorField(res: *PGresult, fieldcode: c_int) -> *c_char;

    // extern int	PQntuples(const PGresult *res);
    fn PQntuples(res: *PGresult) -> c_int;

    // extern int	PQnfields(const PGresult *res);
    fn PQnfields(res: *PGresult) -> c_int;

    // extern int	PQbinaryTuples(const PGresult *res);
    fn PQbinaryTuples(res: *PGresult) -> c_int;

    // extern char *PQfname(const PGresult *res, int field_num);
    fn PQfname(res: *PGresult, field_num: c_int ) -> *c_char;

    // extern int	PQfnumber(const PGresult *res, const char *field_name);
    fn PQfnumber(res: *PGresult, field_name: *c_char) -> c_int;

    // extern Oid	PQftable(const PGresult *res, int field_num);
    fn PQftable(res: *PGresult, field_num: c_int) -> Oid;

    // extern int	PQftablecol(const PGresult *res, int field_num);
    fn PQftablecol(res: *PGresult, field_num: c_int) -> c_int;

    // extern int	PQfformat(const PGresult *res, int field_num);
    fn PQfformat(res: *PGresult, field_num: c_int) -> c_int;

    // extern Oid	PQftype(const PGresult *res, int field_num);
    fn PQftype(res: *PGresult, field_num: c_int) -> Oid;

    // extern int	PQfsize(const PGresult *res, int field_num);
    fn PQfsize(res: *PGresult, field_num: c_int) -> c_int;

    // extern int	PQfmod(const PGresult *res, int field_num);
    fn PQfmod(res: *PGresult, field_num: c_int) -> c_int;

    // extern char *PQcmdStatus(PGresult *res);
    fn PQcmdStatus(res: *PGresult) -> *c_char;

    // extern char *PQoidStatus(const PGresult *res);	/* old and ugly */
    fn PQoidStatus(res: *PGresult) -> *c_char; // old and ugly

    // extern Oid	PQoidValue(const PGresult *res);	/* new and improved */
    fn PQoidValue(res: *PGresult) -> Oid;	// new and improved

    // extern char *PQcmdTuples(PGresult *res);
    fn PQcmdTuples(res: *PGresult) -> *c_char;

    // extern char *PQgetvalue(const PGresult *res, int tup_num, int field_num);
    fn PQgetvalue(res: *PGresult,  tup_num: c_int, field_num: c_int) -> *c_char;

    // extern int	PQgetlength(const PGresult *res, int tup_num, int field_num);
    fn PQgetlength(res: *PGresult, tup_num: c_int, field_num: c_int) -> c_int;

    // extern int	PQgetisnull(const PGresult *res, int tup_num, int field_num);
    fn PQgetisnull(res: *PGresult, tup_num: c_int, field_num: c_int) -> c_int;

    fn PQnparams(res: *PGresult) -> c_int;

    // extern Oid	PQparamtype(const PGresult *res, int param_num);
    fn PQparamtype(res: *PGresult,  param_num: c_int) -> Oid;

    // ---------------------------------------------------------------------
    // Large-object access routines
    // /* === in fe-lobj.c === */

    // extern int	lo_open(PGconn *conn, Oid lobjId, int mode);
    fn lo_open(conn: *PGconn, lobjId: Oid, mode: c_int) -> c_int;

    // extern int	lo_close(PGconn *conn, int fd);
    fn lo_close(conn: *PGconn, fd: c_int) -> c_int;

    // extern int	lo_read(PGconn *conn, int fd, char *buf, size_t len);
    fn lo_read(conn: *PGconn, fd: c_int, buf: *c_char, len: size_t) -> c_int;

    // extern int	lo_write(PGconn *conn, int fd, const char *buf, size_t len);
    fn lo_write(conn: *PGconn, fd: c_int, buf: *c_char, len: size_t) -> c_int;

    // extern int	lo_lseek(PGconn *conn, int fd, int offset, int whence);
    fn lo_lseek(conn: *PGconn, fd: c_int, offset: c_int, whence: c_int) -> c_int;

    // extern Oid	lo_creat(PGconn *conn, int mode);
    fn lo_creat(conn: *PGconn, mode: c_int) -> Oid;

    // extern Oid	lo_create(PGconn *conn, Oid lobjId);
    fn lo_create(conn: *PGconn, lobjId: Oid) -> Oid;

    // extern int	lo_tell(PGconn *conn, int fd);
    fn lo_tell(conn: *PGconn, fd: c_int) -> c_int;

    // extern int	lo_truncate(PGconn *conn, int fd, size_t len);
    fn lo_truncate(conn: *PGconn, fd: c_int,  len: size_t) -> c_int;

    // extern int	lo_unlink(PGconn *conn, Oid lobjId);
    fn lo_unlink(conn: *PGconn, lobjId: Oid) -> c_int;

    // extern Oid	lo_import(PGconn *conn, const char *filename);
    fn lo_import(conn: *PGconn, filename: *c_char) -> Oid;

    // extern Oid	lo_import_with_oid(PGconn *conn, const char *filename, Oid lobjId);
    fn lo_import_with_oid(conn: *PGconn, filename: *c_char, lobjId: Oid) -> Oid;

    // extern int	lo_export(PGconn *conn, Oid lobjId, const char *filename);
    fn lo_export(conn: *PGconn, lobjId: Oid, filename: *c_char) -> c_int;

    fn PQlibVersion() -> c_int;

    // ---------------------------------------------------------------------
    // Simple synchronous query

    // extern PGresult *PQexec(PGconn *conn, const char *query);
    fn PQexec(conn: *PGconn, query: *c_char) -> *PGresult;

    // extern PGresult *PQexecParams(PGconn *conn,
    //                            const char *command,
    //                            int nParams,
    //                            const Oid *paramTypes,
    //                            const char *const * paramValues,
    //                            const int *paramLengths,
    //                            const int *paramFormats,
    //                            int resultFormat);
    fn PQexecParams(conn: *PGconn,
                    command: *c_char,
                    nParams: c_int,
                    paramTypes: *Oid,
                    paramValues: *c_char,
                    paramLengths: *c_int,
                    paramFormats: *c_int,
                    resultFormat: c_int) -> *PGresult;

    // extern PGresult *PQprepare(PGconn *conn, const char *stmtName,
    //                         const char *query, int nParams,
    //                         const Oid *paramTypes);
    fn PQprepare(conn: *PGconn,
                 stmtName: *c_char,
                 query: *char,
                 nParams: c_int,
                 paramTypes: *Oid) -> *PGresult;


    // extern PGresult *PQexecPrepared(PGconn *conn,
    //                              const char *stmtName,
    //                              int nParams,
    //                              const char *const * paramValues,
    //                              const int *paramLengths,
    //                              const int *paramFormats,
    //                              int resultFormat);
    fn PQexecPrepared(conn: *PGconn,
                      stmtName: *char,
                      nParams: c_int,
                      paramValues: **c_char, // assuming (const char *const *)=>(**c_char)
                      paramLengths: *c_int,
                      paramFormats: *c_int,
                      resultFormat: c_int) -> *PGresult;

    // ---------------------------------------------------------------------
    // === in fe-misc.c === */

    // /* Determine length of multibyte encoded char at *s */
    // extern int	PQmblen(const char *s, int encoding);
    fn PQmblen(s: *c_char, encoding: c_int) -> c_int;

    // /* Determine display length of multibyte encoded char at *s */
    // extern int	PQdsplen(const char *s, int encoding);
    fn PQdsplen(s: *c_char, encoding: c_int) -> c_int;

    // /* Get encoding id from environment variable PGCLIENTENCODING */
    // extern int	PQenv2encoding(void);
    fn PQenv2encoding() -> c_int;

    // /* === in fe-auth.c === */

    // extern char *PQencryptPassword(const char *passwd, const char *user);
    fn PQencryptPassword(passwd: *c_char, user: *c_char) -> *c_char;

    // /* === in encnames.c === */

    // extern int	pg_char_to_encoding(const char *name);
    fn pg_char_to_encoding(name: *c_char) -> c_int;

    // extern const char *pg_encoding_to_char(int encoding);
    fn pg_encoding_to_char(encoding: c_int) -> *c_char;
    
    // extern int	pg_valid_server_encoding_id(int encoding);
    fn pg_valid_server_encoding_id(encoding: c_int) -> c_int;


    // /* Describe prepared statements and portals */
    // extern PGresult *PQdescribePrepared(PGconn *conn, const char *stmt);
    fn PQdescribePrepared(conn: *PGconn, stmt: *c_char) -> *PGresult;

    // extern PGresult *PQdescribePortal(PGconn *conn, const char *portal);
    fn PQdescribePortal(conn: *PGconn, portal: *c_char) -> *PGresult;

    // extern int	PQsendDescribePrepared(PGconn *conn, const char *stmt);
    fn PQsendDescribePrepared(conn: *PGconn, stmt: *c_char) -> c_int;

    // extern int	PQsendDescribePortal(PGconn *conn, const char *portal);
    fn PQsendDescribePortal(conn: *PGconn, portal: *char) -> c_int;

    // /* Delete a PGresult */
    // extern void PQclear(PGresult *res);
    fn PQclear(res: *PGresult) -> ();

    // /* For freeing other alloc'd results, such as PGnotify structs */
    // extern void PQfreemem(void *ptr); // assume this won't be needed

    // /* needed for backward compatibility.  bjm 2003-03-24 */
    // #define PQfreeNotify(ptr) PQfreemem(ptr) // assume this won't be needed.

    //     /* Error when no password was given. */
    //     /* Note: depending on this is deprecated; use PQconnectionNeedsPassword(). */
    //     #define PQnoPasswordSupplied	"fe_sendauth: no password supplied\n"


    // ---------------------------------------------------------------------
    // /* Create and manipulate PGresults */

    // extern PGresult *PQmakeEmptyPGresult(PGconn *conn, ExecStatusType status);
    fn PQmakeEmptyPGresult(conn: *PGconn, status: ExecStatusType) -> *PGresult;

    // extern PGresult *PQcopyResult(const PGresult *src, int flags);
    fn PQcopyResult(src: *PGresult, flags: c_int) -> *PGresult;

    // extern int	PQsetResultAttrs(PGresult *res, int numAttributes, PGresAttDesc *attDescs);
    fn PQsetResultAttrs(res: *PGresult, numAttributes: c_int, attDescs: *PGresAttDesc) -> c_int;

    // extern void *PQresultAlloc(PGresult *res, size_t nBytes);
    fn PQresultAlloc(res: *PGresult, nBytes: size_t) -> *c_void;

    // extern int	PQsetvalue(PGresult *res, int tup_num, int field_num, char *value, int len);
    fn PQsetvalue(res: *PGresult, 
                  tup_num: c_int, 
                  field_num: c_int, 
                  value: *c_char, 
                  len: c_int) -> c_int;
    
    // /* Quoting strings before inclusion in queries. */
    // extern size_t PQescapeStringConn(PGconn *conn,
    //                               char *to, const char *from, size_t length,
    //                               int *error);
    fn PQescapeStringConn(conn: *PGconn,
                          to: *char, 
                          from: *char, 
                          length: size_t,
                          error: *int) -> size_t;

    // extern char *PQescapeLiteral(PGconn *conn, const char *str, size_t len);
    fn PQescapeLiteral(conn: *PGconn, str: *char, len: size_t) -> *c_char;
    
    // extern char *PQescapeIdentifier(PGconn *conn, const char *str, size_t len)
    fn PQescapeIdentifier(conn: *PGconn, str: *char, len: size_t) -> *c_char;
    
    // extern unsigned char *PQescapeByteaConn(PGconn *conn,
    //                                      const unsigned char *from, size_t from_length,
    //                                      size_t *to_length);
    fn PQescapeByteaConn(conn: *PGconn,
                         from: *c_uchar,
                         from_length: size_t,
                         to_length : *size_t) -> *c_uchar;
    
    // extern unsigned char *PQunescapeBytea(const unsigned char *strtext,
    //                                    size_t *retbuflen);
    fn PQunescapeBytea(strtext: *c_uchar,
                       retbuflen: *size_t) -> c_uchar;


    // /* Get the OpenSSL structure associated with a connection. Returns NULL for
    // * unencrypted connections or if any other TLS library is in use. */
    // extern void *PQgetssl(PGconn *conn);
    fn PQgetssl(conn: *PGconn) -> *c_void;

    // /* Tell libpq whether it needs to initialize OpenSSL */
    // extern void PQinitSSL(int do_init);
    fn PQinitSSL(do_init: c_int) -> c_void;

    // /* More detailed way to tell libpq whether it needs to initialize OpenSSL */
    // extern void PQinitOpenSSL(int do_ssl, int do_crypto);
    fn PQinitOpenSSL(do_ssl: c_int, do_crypto: c_int) -> c_void;

    // /* Set verbosity for PQerrorMessage and PQresultErrorMessage */
    // extern PGVerbosity PQsetErrorVerbosity(PGconn *conn, PGVerbosity verbosity);
    fn PQsetErrorVerbosity(conn: *PGconn, verbosity: PGVerbosity) -> PGVerbosity;

    // /* Enable/disable tracing */
    // extern void PQtrace(PGconn *conn, FILE *debug_port);
    fn PQtrace(conn: *PGconn, debug_port: *FILE) -> c_void;

    // extern void PQuntrace(PGconn *conn);
    fn PQuntrace(conn: *PGconn) -> c_void;

    
    // /* Override default notice handling routines */
    // extern PQnoticeReceiver PQsetNoticeReceiver(PGconn *conn,
    //                                          PQnoticeReceiver proc,
    //                                          void *arg);
    fn PQsetNoticeReceiver(conn: *PGconn,
                           proc: PQnoticeReceiver,
                           arg: *c_void) -> PQnoticeReceiver;

    // extern PQnoticeProcessor PQsetNoticeProcessor(PGconn *conn,
    //                                            PQnoticeProcessor proc,
    //                                            void *arg);
    fn PQsetNoticeProcessor(conn: *PGconn,
                            proc: PQnoticeProcessor,
                            arg: *c_void) -> PQnoticeProcessor;


    // /* Interface for multiple-result or asynchronous queries */
    // extern int	PQsendQuery(PGconn *conn, const char *query);
    fn PQsendQuery(conn: *PGconn, query: *c_char) -> c_int;

    // extern int PQsendQueryParams(PGconn *conn,
    //                           const char *command,
    //                           int nParams,
    //                           const Oid *paramTypes,
    //                           const char *const * paramValues,
    //                           const int *paramLengths,
    //                           const int *paramFormats,
    //                           int resultFormat);
    fn PQsendQueryParams(conn: *PGconn,
                         command: *c_char,
                         nParams: c_int,
                         paramTypes: *Oid,
                         paramValues: **c_char,
                         paramLengths: *c_int,
                         paramFormats: *c_int,
                         resultFormat: c_int) -> c_int;

    // extern int PQsendPrepare(PGconn *conn, const char *stmtName,
    //                       const char *query, int nParams,
    //                       const Oid *paramTypes);
    fn PQsendPrepare(conn: *PGconn, 
                     stmtName: *c_char,
                     query: *c_char,
                     nParams: c_int,
                     paramTypes: *Oid) -> c_int;

    // extern int PQsendQueryPrepared(PGconn *conn,
    //                             const char *stmtName,
    //                             int nParams,
    //                             const char *const * paramValues,
    //                             const int *paramLengths,
    //                             const int *paramFormats,
    //                             int resultFormat);
    fn PQsendQueryPrepared(conn: *PGconn,
                           stmtName: *c_char,
                           nParams: c_int,
                           paramValues: **c_char,
                           paramLengths: *c_int,
                           paramFormats: *c_int,
                           resultFormat: c_int) -> c_int;

    // extern PGresult *PQgetResult(PGconn *conn);
    fn PQgetResult(conn: *PGconn) -> *PGresult;

    // /* Routines for managing an asynchronous query */
    // extern int	PQisBusy(PGconn *conn);
    fn PQisBusy(conn: *PGconn) -> c_int;
    
    // extern int	PQconsumeInput(PGconn *conn);
    fn PQconsumeInput(conn: *PGconn) -> c_int;

    // /* LISTEN/NOTIFY support */
    // extern PGnotify *PQnotifies(PGconn *conn);
    fn PQnotifies(conn: *PGconn) -> *PGnotify;

    // /* Routines for copy in/out */
    // extern int	PQputCopyData(PGconn *conn, const char *buffer, int nbytes);
    fn PQputCopyData(conn: *PGconn, buffer: *c_char, nbytes: c_int) -> c_int;

    // extern int	PQputCopyEnd(PGconn *conn, const char *errormsg);
    fn PQputCopyEnd(conn: *PGconn, errormsg: *c_char) -> c_int;

    // extern int	PQgetCopyData(PGconn *conn, char **buffer, int async);
    fn PQgetCopyData(conn: *PGconn, buffer: **c_char, async: c_int) -> c_int;

    // /* Deprecated routines for copy in/out */
    // extern int	PQgetline(PGconn *conn, char *string, int length);
    fn PQgetline(conn: *PGconn, string: *c_char, length: c_int) -> c_int;

    // extern int	PQputline(PGconn *conn, const char *string);
    fn PQputline(conn: *PGconn, string: *c_char) -> c_int;

    // extern int	PQgetlineAsync(PGconn *conn, char *buffer, int bufsize);
    fn PQgetlineAsync(conn: *PGconn, buffer: *c_char, bufsize: c_int) -> c_int;

    // extern int	PQputnbytes(PGconn *conn, const char *buffer, int nbytes);
    fn PQputnbytes(conn: *PGconn, buffer: *c_char, nbytes: c_int) -> c_int;

    // extern int	PQendcopy(PGconn *conn);
    fn PQendcopy(conn: *PGconn) -> c_int;

    // /* Set blocking/nonblocking connection to the backend */
    // extern int	PQsetnonblocking(PGconn *conn, int arg);
    fn PQsetnonblocking(conn: *PGconn, arg: c_int) -> c_int;

    // extern int	PQisnonblocking(const PGconn *conn);
    fn PQisnonblocking(conn: *PGconn) -> c_int;

    // extern int	PQisthreadsafe(void);
    fn PQisthreadsafe() -> c_int; // assume removing void is ok.

    // extern PGPing PQping(const char *conninfo);
    fn PQping(conninfo: *c_char) -> PGPing;

    // extern PGPing PQpingParams(const char **keywords,
    //                         const char **values, int expand_dbname);
    fn PQpingParams(keywords: **c_char,
                    values: **c_char, 
                    expand_dbname: c_int) -> PGPing;

    // /* Force the write buffer to be written (or at least try) */
    // extern int	PQflush(PGconn *conn);
    fn PQflush(conn: *PGconn) -> c_int;
}

// ------------------------------------------------------------------
class Conn {
    priv {
        let connstr: str;
        let conn: *PGconn;
    }

    new(connstr: str) {
        self.connstr = copy connstr;
        self.conn = str::as_c_str(connstr, pq::PQconnectdb);
    }

    // ------------------------------------------------------------------
    fn Status() -> str {
        ret alt pq::PQstatus(self.conn) {
          CONNECTION_OK { "CONNECTION_OK" }
          CONNECTION_BAD { "CONNECTION_BAD" }
          CONNECTION_STARTED { "CONNECTION_STARTED" }
          CONNECTION_MADE { "CONNECTION_MADE		" }
          CONNECTION_AWAITING_RESPONSE { "CONNECTION_AWAITING_RESPONSE" }
          CONNECTION_AUTH_OK { "CONNECTION_AUTH_OK" }
          CONNECTION_SETENV { "CONNECTION_SETENV" }
          CONNECTION_SSL_STARTUP { "CONNECTION_SSL_STARTUP" }
          CONNECTION_NEEDED  { "CONNECTION_NEEDED" }
        }
    }

    fn ResetStart() -> int {
        ret pq::PQresetStart(self.conn) as int;
    }

    fn LibVersion() -> int {
        pq::PQlibVersion() as int
    }

    fn ServerVersion() -> int {
        pq::PQserverVersion(self.conn) as int
    }

    fn ProtocolVersion() -> int {
        pq::PQprotocolVersion(self.conn) as int
    }

    unsafe fn Db() -> str {
        unsafe::from_c_str(pq::PQdb(self.conn))
    }

    unsafe fn User() -> str {
        unsafe::from_c_str(pq::PQuser(self.conn))
    }
    unsafe fn Pass() -> str {
        unsafe::from_c_str(pq::PQpass(self.conn))
    }
    unsafe fn Host() -> str {
        unsafe::from_c_str(pq::PQhost(self.conn))
    }
    unsafe fn Port() -> str {
        unsafe::from_c_str(pq::PQport(self.conn))
    }
    unsafe fn Tty() -> str {
        unsafe::from_c_str(pq::PQtty(self.conn))
    }
    unsafe fn Options() -> str {
        unsafe::from_c_str(pq::PQoptions(self.conn))
    }

    // ------------------------------------------------------------------
    fn Exec(query: str) -> Result {
        //fn PQexec(conn: *PGconn, query: *c_char) -> *PGresult;
        let r = str::as_c_str(query, bind pq::PQexec(self.conn, _));
        Result(r)
    }

}

// ------------------------------------------------------------------
// #[test]
fn Connect() -> Conn {
    let connstr = "host=localhost \
                   port=5432 \
                   user=rustuser \
                   dbname=rust_test_db \
                   password=rustpass \
                   sslmode=disable";

    let conn = Conn(connstr);
    log(error, conn.Status());
    log(error, conn.LibVersion());
    log(error, conn.ServerVersion());
    log(error, conn.ProtocolVersion());
    log(error, conn.Db());
    log(error, conn.User());
    log(error, conn.Pass());
    log(error, conn.Host());
    log(error, conn.Port());
    log(error, conn.Tty());
    log(error, conn.Options());
    //log(error, conn.ResetStart());
    ret conn
}

// ------------------------------------------------------------------
class Result {
    let res: *PGresult;
    new(r: *PGresult){
        self.res = r;
    }

    fn Status() -> ExecStatusType {
        ret pq::PQresultStatus(self.res)
    }

    fn StatusAsStr() -> str {
        alt self.Status() {
          PGRES_EMPTY_QUERY {"PGRES_EMPTY_QUERY"}
          PGRES_COMMAND_OK {"PGRES_COMMAND_OK"}
          PGRES_TUPLES_OK {"PGRES_TUPLES_OK"}
          PGRES_COPY_OUT {"PGRES_COPY_OUT"}
          PGRES_COPY_IN {"PGRES_COPY_IN"}
          PGRES_BAD_RESPONSE {"PGRES_BAD_RESPONSE"}
          PGRES_NONFATAL_ERROR {"PGRES_NONFATAL_ERROR"}
          PGRES_FATAL_ERROR {"PGRES_FATAL_ERROR"}
          PGRES_COPY_BOTH {"PGRES_COPY_BOTH"}
        }
    }

    // fn PQresStatus(status: ExecStatusType) -> c_char {
    //     // extern char *PQresStatus(ExecStatusType status);
    // }

    unsafe fn ErrorMessage() -> str {
        unsafe::from_c_str(pq::PQresultErrorMessage(self.res))
    }

    unsafe fn ErrorField(fieldcode: int) -> str {
        let err = pq::PQresultErrorField(self.res, fieldcode as c_int);
        if ptr::is_null(err) {
            "No error found"
        } else {
            unsafe::from_c_str(err)
        }
    }

    fn Ntuples() -> int {
        pq::PQntuples(self.res) as int
    }

    fn Nfields() -> int {
        pq::PQnfields(self.res) as int
    }

    fn BinaryTuples() -> int {
        pq::PQbinaryTuples(self.res) as int
    }

    unsafe fn Fname(field_num: int) -> str {
        unsafe::from_c_str(pq::PQfname(self.res, field_num as c_int))
    }

    fn Fnumber(field_name: str) -> int {
        str::as_c_str(field_name, bind pq::PQfnumber(self.res, _)) as int
    }

    fn Ftable(field_num: int) -> Oid {
        pq::PQftable(self.res, field_num as c_int)
    }

    fn Ftablecol(field_num: int) -> int {
        pq::PQftablecol(self.res, field_num as c_int) as int
    }

    fn Fformat(field_num: int) -> int {
        pq::PQfformat(self.res, field_num as c_int) as int
    }

    fn Ftype(field_num: int) -> Oid {
        pq::PQftype(self.res, field_num as c_int)
    }
    fn Fsize(field_num: int) -> int {
        pq::PQfsize(self.res, field_num as c_int) as int
    }
    fn Fmod(field_num: int) -> int {
        pq::PQfmod(self.res, field_num as c_int) as int
    }
    unsafe fn CmdStatus() -> str {
        unsafe::from_c_str(pq::PQcmdStatus(self.res))
    }
    // fn PQoidStatus(res: PGresult) -> c_char; { // old and ugly {
    //     // extern char *PQoidStatus(const PGresult *res);	/* old and ugly */
    // }
    fn OidValue() -> Oid {          // new and improved
        pq::PQoidValue(self.res)	
    }
    unsafe fn CmdTuples() -> str {
        unsafe::from_c_str(pq::PQcmdTuples(self.res))
    }
    unsafe fn GetValue(tup_num: int, field_num: int) -> str {
        unsafe::from_c_str(
            pq::PQgetvalue(self.res,
                           tup_num as c_int,
                           field_num as c_int))
    }
    fn GetLength(tup_num: int, field_num: int) -> int {
        pq::PQgetlength(self.res,
                        tup_num as c_int,
                        field_num as c_int) as int
    }
    fn GetIsNull(tup_num: int, field_num: int) -> bool {
        pq::PQgetisnull(self.res, tup_num as c_int, field_num as c_int) as bool
    }
    fn Nparams() -> int {
        pq::PQnparams(self.res) as int
    }
    //     // extern Oid	PQparamtype(const PGresult *res, int param_num);
    // }

}

#[test]
fn ResultTest() {
    let conn = Connect();
    let res = conn.Exec("select * from movie");
    log(error, res.Status());
    log(error, "status:    " + res.StatusAsStr());
    log(error, "error msg: " + res.ErrorMessage());
    log(error, res.Ntuples());
    log(error, res.Nfields());
    log(error, res.BinaryTuples());
    log(error, res.Fname(1));
    log(error, res.Ftable(1));
    log(error, res.Ftablecol(1));
    log(error, res.Fformat(1));
    log(error, res.Fsize(1));
    log(error, res.Fmod(1));
    log(error, "CmdStatus: " + res.CmdStatus());
    log(error, res.OidValue());
    log(error, res.CmdTuples());
    log(error, res.GetValue(1,1));
    log(error, res.GetLength(1,1));
    log(error, res.GetIsNull(1,1));
    log(error, res.Nparams());
    log(error, res.ErrorField(2));
}


// #define PQsetdb(M_PGHOST,M_PGPORT,M_PGOPT,M_PGTTY,M_DBNAME)  \
//  PQsetdbLogin(M_PGHOST, M_PGPORT, M_PGOPT, M_PGTTY, M_DBNAME, NULL, NULL)
