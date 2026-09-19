//
//  HomeworkCache.swift
//  Libellule
//
//  Created by Jules on 12/09/2026.
//

import Foundation
import SwiftData
import LibelluleKit

@Model
final class CachedHomework {
    var date: Date
    var fetchedAt: Date
    @Relationship(deleteRule: .cascade) var items: [CachedHomeworkItem]
    
    init(date: Date, homework: LibelluleKit.Homework) {
        var items = [CachedHomeworkItem]()
        for item in homework.items {
            items.append(CachedHomeworkItem(item: item))
        }
        
        self.date = date
        self.fetchedAt = .now
        self.items = items
    }
}

@Model
final class CachedHomeworkItem {
    var id: String
    var subjectName: String
    var contents: String
    var done: Bool
    var creation: Date
    var due: Date
    var color: String
    
    init(id: String, subjectName: String, contents: String, done: Bool, creation: Date, due: Date, color: String) {
        self.id = id
        self.subjectName = subjectName
        self.contents = contents
        self.done = done
        self.creation = creation
        self.due = due
        self.color = color
    }
    
    init(item: LibelluleKit.HomeworkItem) {
        self.id = item.id
        self.subjectName = item.subject.name
        self.contents = item.description
        self.done = item.done
        self.creation = FFIDate.date.date(from: item.creation)!
        self.due = FFIDate.date.date(from: item.due)!
        self.color = item.background
    }
}
