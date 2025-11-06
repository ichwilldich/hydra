# Deployment

## Configuration Options

General Options:

- Name: The display name for the deployment.
- Namespace: The Kubernetes namespace where the deployment will be created. (only Kubernetes)
- Database Version: The version of postgres to deploy.

Resource Options:

- Replicas: The number of instances to deploy.
- Storage Size: The amount of storage allocated for the deployment per instance (in MB). (only Kubernetes)
- Storage Class: The storage class to use for the deployment. (only Kubernetes)
- CPU Requests: The amount of CPU requested for each instance (in millicores).
- CPU Limits: The maximum amount of CPU allowed for each instance (in millicores).
- Memory Requests: The amount of memory requested for each instance (in MB).
- Memory Limits: The maximum amount of memory allowed for each instance (in MB).

Backup Options:

- Backup Name: The name of the backup configuration.
- Backup Schedule: The cron schedule for automatic backups.
- Backup Retention: The number of backups to retain.
- Backup Location: The location where backups will be stored.

Advanced Options:

- External Access: Enable or disable external access to the deployment.
- Extra Database Engine Parameters: Additional parameters to configure the database engine.
- Monitoring: Enable or disable monitoring for the deployment.

storage ref:
docker: - host file path (must exist) - file upload
kubernetes: - file upload - config map ref - secret ref

!
ssl
ssl_ca_file (storage ref)
ssl_cert_file (storage ref)
ssl_key_file (storage ref)
wal_level = replica (restart)
allow_alter_system (default to off)

primary:
hot_standby = on (restart)

sending:
max_wal_senders = replica count + small number like 10% (restart)
max_replication_slots = replica count + small number like 10% (restart)

standby:
primary_conninfo (use env)
primary_slot_name (just some name)
sync_replication_slots = true
hot_standby = on (restart)

stats TODO (replication lag monitoring)
archiving for replication TODO
failover TODO
