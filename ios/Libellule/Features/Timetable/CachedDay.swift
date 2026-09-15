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
    var color: String
    var cancelled: Bool
    
    init(lesson: Lesson) {
        self.id = lesson.id
        self.start = lesson.start
        self.end = lesson.end
        self.subjectName = lesson.subject.name
        self.teachers = lesson.teachers
        self.rooms = lesson.locations.map(\.name)
        self.groups = lesson.groups.map(\.name)
        self.color = lesson.color
        self.cancelled = lesson.cancelled
    }
}
