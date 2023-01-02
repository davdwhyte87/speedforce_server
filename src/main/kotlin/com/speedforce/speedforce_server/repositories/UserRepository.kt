package com.speedforce.speedforce_server.repositories

import com.speedforce.speedforce_server.model.User
import org.springframework.data.mongodb.repository.MongoRepository

interface UserRepository :MongoRepository<User,String>{
}