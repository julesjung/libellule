//
//  CachedMenu.swift
//  Libellule
//
//  Created by Jules on 15/09/2026.
//

import Foundation
import SwiftData
import LibelluleKit

@Model
final class CachedMenu {
    var date: Date
    var fetchedAt: Date
    var lunch: CachedMeal?
    var dinner: CachedMeal?
    
    init(date: Date, menu: LibelluleKit.Menu) {
        self.date = date
        self.fetchedAt = .now
        
        if let lunch = menu.lunch {
            self.lunch = CachedMeal(meal: lunch)
        }
        
        if let dinner = menu.dinner {
            self.dinner = CachedMeal(meal: dinner)
        }
    }
}

@Model
final class CachedMeal {
    var starter: CachedCourse?
    var main: CachedCourse?
    var trimmings: CachedCourse?
    var dairies: CachedCourse?
    var desserts: CachedCourse?
    
    init(meal: LibelluleKit.Meal) {
        if let starter = meal.starter {
            self.starter = CachedCourse(course: starter)
        }
        
        if let main = meal.main {
            self.main = CachedCourse(course: main)
        }
        
        if let trimmings = meal.trimmings {
            self.trimmings = CachedCourse(course: trimmings)
        }
        
        if let dairies = meal.dairies {
            self.dairies = CachedCourse(course: dairies)
        }
        
        if let desserts = meal.desserts {
            self.desserts = CachedCourse(course: desserts)
        }
    }
}

@Model
final class CachedCourse {
    var food: [String]
    
    init(course: LibelluleKit.Course) {
        self.food = course.food.map { $0.label }
    }
}
