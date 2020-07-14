# cli
CLI for working with Erinite

* `erin new <project-name>`
  Create a new Erinite project
* `erin add module <modules>`
  Add one or more modules to the project. Backend projects only.
  
  `<modules>` is one or more of:
    * `--webserver`
    * `--database <db>`, where `<db>` is one or more of: `sql`, `redis`
* `erin add boundary <bouanry-name> <options>`
  Add a new boundary to the project. Backend projects only.
  
  Use `<options>` to include protocol implementations.
  
  `<options>` is zero or more of: `--sql`, `--redis`, `--queue`
* `erin add service <service-name> <options>`
  Add a new service to the project. Backend projects only.
  
  If `<options>` includes `--with-boundary` or `-b`, then a boundary of the same name as the service is imported.
  
  Other available `<options>` are: `--router`, `--db` and `--queue <queue-name(s)>`
* `erin add feature <feature-name>`
  Add a new feature to the project. Frontend projects only.
