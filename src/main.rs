use graphql_client::GraphQLQuery;

use accept_topic_suggestion::AcceptTopicSuggestionInput;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/query.graphql",
    schema_path = "src/schema.graphql",
    variables_derives = "Debug"
)]
struct AcceptTopicSuggestion;

fn main() {
    let variables = accept_topic_suggestion::Variables {
        input: AcceptTopicSuggestionInput {
            client_mutation_id: None,
            name: "Hello".to_string(),
            repository_id: "World".to_string(),
        },
    };
    dbg!(variables);
}
