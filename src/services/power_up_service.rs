use std::borrow::Borrow;
use std::fmt::{Debug};
use actix_web::App;
use mongodb::bson::doc;
use std::error::Error;
// use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use mongodb::results::{InsertOneResult, UpdateResult};
use crate::models::test_record::TestRecord;
use mongodb::bson::extjson::de::Error::DeserializationError;
use futures::stream::TryStreamExt;
use mongodb::change_stream::event::ResumeToken;
use mongodb::error::{ErrorKind};
use r2d2_mongodb::mongodb::Error::OperationError;
use crate::models::power_up::{PlayerPowerUp, PowerUp, PowerUpType};
use crate::models::test_data::TestData;

const COLLECTION_NAME:&str = "PowerUp";


pub struct PowerUpService {

}



impl PowerUpService{
    pub async fn get_by_id(db:&Database, id:String)->Result<Option<TestData>, Box<dyn Error>>{
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let collection = db.collection::<TestData>(COLLECTION_NAME);
        let user_detail = collection.find_one(filter, None).await;
        let user_detail = match user_detail {
            Ok(user_detail)=>{
                user_detail
            },
            Err(err)=>{return Err(err.into())}
        };
        Ok(user_detail)
    }

    pub async fn create(db: &Database, power_up: &PowerUp) -> Result<InsertOneResult, Box<dyn Error>> {
        // Get a handle to a collection in the database.
        let collection = db.collection::<PowerUp>(COLLECTION_NAME);
        let res_diag =collection.insert_one(power_up, None).await;
        match res_diag {
            Ok(res_)=>{return Ok(res_)},
            Err(err)=>{return Err(err.into())}
        }
    }
    pub async fn create_player_powerup(db: &Database, power_up: &PlayerPowerUp) -> Result<InsertOneResult, Box<dyn Error>> {
        // Get a handle to a collection in the database.
        let collection = db.collection::<PlayerPowerUp>(COLLECTION_NAME);

        // check if the player already has an existing power up
        let user_power_up = PowerUpService::get_user_power_up(db, power_up.user_email.to_owned().to_string(), match power_up.power_up_type {
            PowerUpType::Phasing => {"Phasing".to_string()}
            PowerUpType::Blast => {"Blast".to_string()}
            PowerUpType::SlowMotion => {"SlowMotion".to_string()}
        }).await;
        let user_power_up = match user_power_up {
            Ok(user_power_up)=> {return Err(Box::from("There is already a player power up"))},
            Err(err)=>{

            }
        };

        let res_diag =collection.insert_one(power_up, None).await;
        match res_diag {
            Ok(res_)=>{return Ok(res_)},
            Err(err)=>{return Err(err.into())}
        }
    }


    pub async fn get_user_power_up(
        db:&Database,
        email:String,
        power_up_type:String
    )
        -> Result<PlayerPowerUp, Box<dyn Error>>
    {
        let filter = doc! {"user_email":email, "power_up_type":power_up_type};
        let collection = db.collection::<PlayerPowerUp>(COLLECTION_NAME);
        let mut user_power_up = collection.find_one(filter, None).await.ok().expect("Error getting test data");
        let user_power_up =match user_power_up {
            Some(user_power_up)=>{user_power_up},
            None=>{
              return Err(Box::from("User not found"))
            }
        };
        Ok(user_power_up)
    }



    pub async fn get_player_power_ups(
        db:&Database,
        email:&String
    )
        -> Result<Vec<PlayerPowerUp>, Box<dyn Error>>
    {
        let filter = doc! {"user_email":email};
        let collection = db.collection::<PlayerPowerUp>(COLLECTION_NAME);
        let mut cursor = collection.find(filter, None).await;
        let mut cursor =match cursor {
            Ok(user_power_up)=>{user_power_up},
            Err(err)=>{
                return Err(Box::from("User not found"))
            }
        };
        let mut powerups:Vec<PlayerPowerUp> = Vec::new();

        while let Some(power_up)= match cursor.try_next().await {
            Ok(cursor) => {cursor}
            Err(err) => {return Err(err.into())}
        } {
            powerups.push(power_up);
        }
        Ok(powerups)
    }
    
    pub async fn update(
        db:&Database,
        email:String,
        power_up_type:String,
        mut new_data:&PlayerPowerUp
    )
        ->Result<UpdateResult, Box<dyn Error>>
    {
        let filter = doc! {"user_email":email, "power_up_type":power_up_type};
        let collection = db.collection::<PlayerPowerUp>(COLLECTION_NAME);
        let new_doc = doc! {
            "$set":{
                "amount":new_data.amount.to_owned(),
                "in_game_amount":new_data.in_game_amount.to_owned(),
            }
        };
        let updated_doc = collection.update_one(filter,new_doc, None )
            .await;

        match updated_doc {
            Ok(updated_doc)=>{return Ok(updated_doc)},
            Err(err)=>{
                return Err(err.into())
            }
        }
    }

    // each test record has many related testdata, this function gets all test data for a given
    //test record
    pub async fn get_all_test_data_for_test_record(db:&Database, id:String){

    }
}