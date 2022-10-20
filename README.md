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

### Client


###### darryl.west | 2022.10.19

