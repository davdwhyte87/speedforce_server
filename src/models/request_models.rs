
use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::models::power_up::PowerUpType;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  CreateDiagnosisReq{
    #[validate(length(min=0))]
    pub symptoms:String,
    pub prescription: String,
    pub note: String,
    #[validate(email)]
    pub patient_email:String,
    #[validate(email)]
    pub nurse_email:String
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  UpdateDiagnosisReq{
    #[validate(length(min=1))]
    pub symptoms:String,
    #[validate(length(min=1))]
    pub prescription: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  LoginReq{
    #[validate(email)]
    pub email:String,
    #[validate(length(max=6))]
    pub code:String
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  GetCodeReq{
    #[validate(email)]
    pub email:String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  CreateTestRecordReq{
    #[validate(email)]
    pub nurse_email:String,
    #[validate(email)]
    pub patient_email: String,
    pub note:String
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  UpdateTestDataReq{
    #[validate(length(min=1))]
    pub name:String,
    #[validate(length(min=1))]
    pub result: String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  CreateAccountDetailsReq{

    pub account_name: String,

    pub account_number: String,

    pub bank_name: String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  BuyCoinReq{
    pub amount: String,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  UsePowerUpReq{
    pub power_up_type: PowerUpType,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  BuyPowerUpReq{
    pub power_up_type: PowerUpType,
    pub amount:i32
}



#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  UpdatePlayerRunReq{
    pub distance: i32
}