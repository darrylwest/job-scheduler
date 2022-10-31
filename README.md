# job-scheduler

A redis backed, REST job scheduler written in rust

### Server

#### API

All API keys require valid app and user tokens.  The app token tracks the application stats, the user token tracks the specific user.

* GET /status : returns the job scheduler's status
* GET /jobs/:status : returns job ids with the specified status (new, active, processed, etc) with a limit of 1000 rows
* GET /job/:id : returns the details of the specified job
* POST /job : creates a new job ; returns the job details as provided by the JSON request
* PUT /job/:id : updates a job
* DEL /job/:id : archives a completed job, cancels (if possible) an active/new job


#### Data Model

The Job data model is wrapped by the standard rxkv `Model<T>` that provides, id, version and status. In this context the status values used are:

* New() : when the job is first requested and inserted into the kv store 0 = inserted into db, 128 = queued
* Active() : when the job is executing 0..255 = the job step
* Processed() : when the job completes; value of 0..127 = success, 128..255 = failed 
* Blocked() : if there is an issue that needs to be resolved before the job can complete
* Deleted() : when the job is archived

```bash

struct Job {
    topic Cow<'static, str>,
    description Cow<'static, str>,
    action Cow<'static, str>,
    request_from Cow<'static, str>,
    request_to Cow<'static, str>,
    results Cow<'static, str>,
    log Vec<Cow<'static, str>,
    errors Vec<Cow<'static, str>,
}
```

#### Indexes

Indexes are implement as sets

* jobs : all the current jobs (primary index) excluding deleted
* jobs.new : newly requested jobs
* jobs.active : jobs in process
* jobs.processed : jobs that have been recently completed (prior to delete/archive)
* jobs.blocked : any jobs that are currently blocked

### Client


###### darryl.west | 2022.10.31

