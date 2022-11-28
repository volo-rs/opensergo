use opensergo_proto::opensergo::proto::service_contract::v1::*;

lazy_static::lazy_static! {
    static ref DEFAULT_CLIENT: MetadataServiceClient = MetadataServiceClientBuilder::new(crate::env::get_communication_config().unwrap().endpoint).build();
}

pub async fn report_metadata(
    app_name: String,
    node: Option<Node>,
    service_metadata: Vec<ServiceMetadata>,
) -> anyhow::Result<ReportMetadataReply> {
    let client = DEFAULT_CLIENT.clone();
    let request = ReportMetadataRequest {
        app_name,
        node,
        service_metadata,
    };
    Ok(client.report_metadata(request).await?.into_inner())
}
