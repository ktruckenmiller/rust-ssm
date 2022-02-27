use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{Client, Error};

#[derive(Debug)]
pub struct ParameterItem {
    pub name: String,
    pub value: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let ssm = Client::new(&config);
    let mut token: Option<String> = None;
    let mut items: Vec<ParameterItem> = Vec::new();

    loop {
        let res = ssm.get_parameters_by_path()
            .path("/")
            .recursive(true)
            .set_next_token(token.clone())
            .send()
            .await;
        
        match res {
            Ok(res) => {
                for parameters in res.parameters {
                    for parameter in parameters {
                        items.push(ParameterItem {
                            name: parameter.name.expect("Name is required"),
                            value: parameter.value.expect("Value is required")
                        })
                    }
                }
                if res.next_token == None {
                    break;
                }
                token = res.next_token;
            }
            Err(error) => {
                eprintln!(
                    "[parameters] Error calling ssm:GetParametersByPath. Error: {err}",
                    err = error.to_string()
                );
                break;
            }
        }
        println!("paginating")

    }
    

    for param in items {
        println!("  {:?}", param);
        
    }
    // println!("boston");

    Ok(())
}