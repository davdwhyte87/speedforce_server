package com.speedforce.speedforce_server.model

import org.bson.types.ObjectId
import org.springframework.data.annotation.Id

data class User(
    @Id
    val id:ObjectId = ObjectId.get(),
    val name:String,
    val email:String,
    val authCode:Int,
    val createdAt:String,
    val deviceID:String,
    val location:String,
    val role:Role
)
