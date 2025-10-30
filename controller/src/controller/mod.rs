use std::{pin::pin, sync::Arc, time::Duration};

use centaurus::error::{ErrorReport, Result};
use futures::StreamExt;
use kube::{
  Api, Client, ResourceExt,
  api::ListParams,
  runtime::{Controller, controller::Action, finalizer, watcher::Config},
};
use tracing::{error, info, instrument};

use crate::crd::Document;

mod error;

static DUMMY_FINALIZER: &str = "dummy.example.com/finalizer";

struct Context {
  client: Client,
}

pub async fn run(client: Client) {
  let dummy = Api::<Document>::all(client.clone());

  if let Err(e) = dummy.list(&ListParams::default().limit(1)).await {
    error!("Failed to list Dummy CRD instances: {}", e);
    error!("Ensure that the CRD is applied to the cluster");
    std::process::exit(1);
  }

  let mut stream = pin!(
    Controller::new(dummy, Config::default().any_semantic())
      .shutdown_on_signal()
      .run(reconcile, error_policy, Arc::new(Context { client }))
  );

  info!("Starting controller");
  while let Some(result) = stream.next().await {
    if let Err(e) = result {
      error!("Error processing stream item: {}", e);
    }
  }
}

#[instrument(skip(dummy, ctx))]
async fn reconcile(dummy: Arc<Document>, ctx: Arc<Context>) -> Result<Action> {
  let ns = dummy.namespace().unwrap(); // Dummy is namespaced
  let api: Api<Document> = Api::namespaced(ctx.client.clone(), &ns);

  info!(
    "Reconciling Dummy: {} in namespace {}",
    dummy.name_any(),
    ns
  );

  finalizer(&api, DUMMY_FINALIZER, dummy, |event| async {
    match event {
      finalizer::Event::Apply(dummy) => dummy.reconcile().await,
      finalizer::Event::Cleanup(dummy) => dummy.cleanup().await,
    }
  })
  .await
  .map_err(|e| e.into())
}

fn error_policy(dummy: Arc<Document>, error: &ErrorReport, _ctx: Arc<Context>) -> Action {
  error!(
    "Reconciliation error for Dummy {}: {:?}",
    dummy.name_any(),
    error
  );
  Action::requeue(Duration::from_secs(5 * 60))
}

impl Document {
  #[instrument(skip(self))]
  async fn reconcile(&self) -> Result<Action> {
    info!("Reconciling Document: {}", self.name_any());
    // Add your reconciliation logic here
    Ok(Action::requeue(Duration::from_secs(5 * 60)))
  }

  #[instrument(skip(self))]
  async fn cleanup(&self) -> Result<Action> {
    info!("Cleaning up Document: {}", self.name_any());
    // Add your cleanup logic here
    Ok(Action::requeue(Duration::from_secs(5 * 60)))
  }
}
