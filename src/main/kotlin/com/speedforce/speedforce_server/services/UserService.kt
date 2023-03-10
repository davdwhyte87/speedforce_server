package com.speedforce.speedforce_server.services

import com.speedforce.speedforce_server.model.User
import com.speedforce.speedforce_server.repositories.UserRepository
import org.springframework.data.mongodb.core.MongoTemplate
import org.springframework.data.mongodb.core.query.Criteria
import org.springframework.data.mongodb.core.query.Query

import org.springframework.stereotype.Service


@Service
class UserService(private val _userRepository: UserRepository, private val mongoTemplate: MongoTemplate) {
    var userRepository:UserRepository = _userRepository

    fun findByEMail(email:String) :User?{
        var query = Query().addCriteria(Criteria.where("email").`is`(email))
        return mongoTemplate.findOne(query, User::class.java)
    }
}