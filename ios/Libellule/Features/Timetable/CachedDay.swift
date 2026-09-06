//
//  CachedDay.swift
//  Libellule
//
//  Created by Jules on 31/08/2026.
//

import Foundation
import SwiftData
import LibelluleKit

@Model
final class CachedDay {
    var date: String
    var fetchedAt: Date
    @Relationship(deleteRule: .cascade) var lessons: [CachedLesson]
    
    init(date: String, lessons: [CachedLesson]) {
        self.date = date
        self.fetchedAt = .now
        self.lessons = lessons
    }
}

@Model
final class CachedLesson {
    var id: String
    var start: String
    var end: String
    var subjectName: String
    var teachers: [String]
    var rooms: [String]
    var groups: [String]
    var background: String
    
    init(id: String, start: String, end: String, subjectName: String, teachers: [String], rooms: [String], groups: [String], background: String) {
        self.id = id
        self.start = start
        self.end = end
        self.subjectName = subjectName
        self.teachers = teachers
        self.rooms = rooms
        self.groups = groups
        self.background = background
    }
    
    init(lesson: Lesson) {
        self.id = lesson.id
        self.start = lesson.start
        self.end = lesson.end
        self.subjectName = lesson.subject.name
        self.teachers = lesson.teachers
        self.rooms = lesson.locations.map(\.name)
        self.groups = lesson.groups.map(\.name)
        self.background = lesson.background
    }
}
