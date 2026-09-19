//
//  HomeView.swift
//  Libellule
//
//  Created by Jules on 17/09/2026.
//

import SwiftUI
import SwiftData

struct HomeView: View {
    @Query private var nextDays: [CachedDay]
    
    private var nextLessons: [CachedLesson] {
        let now = FFIDate.time.string(from: .now)
        
        if let today = nextDays.first {
            let lessons = today.lessons.sorted { $0.start < $1.start }
            if let index = lessons.firstIndex(where: { $0.end >= now }) {
                return Array(lessons[index...].prefix(3))
            }
        }
        
        return []
    }
    
    init() {
        let today = FFIDate.date.string(from: .now)
        
        self._nextDays = Query(filter: #Predicate { $0.date == today })
    }
    
    var body: some View {
        NavigationStack {
            Group {
                if !nextLessons.isEmpty {
                    List(nextLessons) { lesson in
                        LessonView(lesson: lesson)
                    }
                }
            }
            .navigationTitle("Bonjour")
        }
    }
}
