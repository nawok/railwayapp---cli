use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/DeploymentRemove.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct DeploymentRemove;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/LoginSessionConsume.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct LoginSessionConsume;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/LoginSessionCreate.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct LoginSessionCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/PluginCreate.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct PluginCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/PluginDelete.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct PluginDelete;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/ProjectCreate.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct ProjectCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/ServiceDomainCreate.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct ServiceDomainCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/mutations/strings/ValidateTwoFactor.graphql",
    response_derives = "Debug, Serialize, Clone"
)]
pub struct ValidateTwoFactor;
