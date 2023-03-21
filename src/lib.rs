use pgx::prelude::*;
use std::env;
use pgx::Uuid;

pgx::pg_module_magic!();


use tonic::{
    metadata::{MetadataValue},
    transport::Channel,
    Request,
    Response
};

use v1::permissions_service_client::PermissionsServiceClient;
use v1::consistency::Requirement;
use v1::{CheckPermissionRequest, CheckPermissionResponse, Consistency, ObjectReference, SubjectReference};


#[cfg(feature = "v1")]
pub mod v1 {
    tonic::include_proto!("authzed.api.v1");
}



#[pg_extern]
#[tokio::main]
async fn has_permission(id: Uuid, subject_id: String) -> bool {
    let res = check_permission_request(id.to_string(), subject_id).await.unwrap();
    info!("We are processing the request now {:?}", res);
    if res == 2 {
        true
    }else {
        false
    }
}

async fn check_permission_request(id: String, subject_id: String) -> Result<i32, Box<dyn std::error::Error>> {
    // Set up the access token for authentication

    //Environment vars
    let access_token = env::var("SPICE_KEY").unwrap_or(String::from("somerandomkeyhere"));
    let service:String = env::var("SPICE_SERVICE").unwrap_or(String::from("http://spice:50051"));
    let resource = "transaction".to_string();
    let subject = "user".to_string();
    let permission = "view".to_string();

    let authorization_header = format!("Bearer {}", access_token);
    let authorization_header_value = MetadataValue::try_from(&authorization_header)?;

    // Set up the channel and client for the gRPC request
    let channel = Channel::from_shared(service).unwrap().connect().await?;
    let mut client = PermissionsServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", authorization_header_value.clone());
        Ok(req)
    });

    // Set up the request message and metadata
    let trsn_resource: Option<ObjectReference> = Some(ObjectReference {
        object_type: resource,
        object_id: id,
    });
    let consistency: Option<Consistency> = Some(Consistency {
        requirement: Some(Requirement::FullyConsistent(true)),
    });
    let trsn_subject: Option<SubjectReference> = Some(SubjectReference {
        object: Some(ObjectReference {
            object_type: subject,
            object_id: subject_id,
        }),
        optional_relation: "".to_string(),
    });

    let request = Request::new(CheckPermissionRequest {
        consistency: consistency,
        resource: trsn_resource,
        permission: permission,
        subject: trsn_subject,
    });

    // Send the request and process the response
    let response: Response<CheckPermissionResponse> = client.check_permission(request).await?;
    let data = response.into_inner();
    println!("Response: {:?}", data.permissionship);

    Ok(data.permissionship)
}

// #[pg_trigger]
// fn process_trigger(trigger: &PgTrigger) -> Result<
//     PgHeapTuple<'_, impl WhoAllocated>,
//     PgHeapTupleError,
//     > {
    
//     let logvar = unsafe {trigger.current()}.expect("Kuna shida kubwa ka inapita hapa");
//     let title_opt = logvar.into_owned().get_by_name("title").unwrap();
//     let title_val = match title_opt {
//         Some(t) => t,
//         None => "Yikes".to_string()
//     };

//     info!("We are processing the trigger now {:?}", title_val);
//     // make_request(title_val);

//     Ok(unsafe{ trigger.current()}.expect("No current Heap tupple"))
// }

// pgx::extension_sql!(
//     r#"
// CREATE TABLE transactions (
//     id uuid NOT NULL PRIMARY KEY,
//     title varchar(50),
//     description text
// );
// CREATE ROLE readwrite;
// GRANT CONNECT ON DATABASE db_dev TO readwrite;
// GRANT USAGE, CREATE ON SCHEMA public TO readwrite;
// GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE transactions TO readwrite;
// GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO readwrite;
// ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE ON SEQUENCES to readwrite;
// CREATE USER mesh WITH PASSWORD 'mesh';
// GRANT readwrite TO mesh;

// ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
// CREATE POLICY viewable_by_id ON transactions FOR SELECT USING ((SELECT has_permission(id, (SELECT current_user))));
// "#,
//     name = "initial_setup",
//     requires = [ process_trigger, has_permission ]
// );



/// This module is required by `cargo pgx test` invocations. 
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
