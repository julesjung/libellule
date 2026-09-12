//
//  DayView.swift
//  Libellule
//
//  Created by Jules on 31/08/2026.
//

import SwiftUI
import SwiftData

struct DayView: View {
    let date: String
    @Query private var days: [CachedDay]
    @Environment(\.syncService) private var sync
    
    init(date: String) {
        self.date = date
        _days = Query(filter: #Predicate<CachedDay> { $0.date == date })
    }
    
    var body: some View {
        Group {
            if let day = days.first {
                LessonList(lessons: day.lessons.sorted { $0.start < $1.start })
            } else {
                ContentUnavailableView("Aucun cours", systemImage: "beach.umbrella")
            }
        }
        .task(id: date) {
            await sync?.refreshDayIfStale(date)
        }
        .refreshable {
            await sync?.refreshDay(date)
        }
    }
}
