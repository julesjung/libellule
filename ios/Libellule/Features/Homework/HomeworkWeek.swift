//
//  HomeworkWeek.swift
//  Libellule
//
//  Created by Jules on 12/09/2026.
//

import SwiftUI
import SwiftData

struct HomeworkWeek: View {
    let date: Date
    @Query private var homeworkWeeks: [CachedHomework]
    @Environment(\.syncService) private var sync
    
    init(date: Date) {
        self.date = date
        _homeworkWeeks = Query(filter: #Predicate<CachedHomework> { $0.date == date })
    }
    
    var body: some View {
        Group {
            if let items = homeworkWeeks.first?.items, !items.isEmpty {
                HomeworkList(items: items)
            } else {
                ContentUnavailableView("Aucun devoir", systemImage: "beach.umbrella")
            }
        }
        .task(id: date) {
            await sync.refreshHomeworkIfStale(date)
        }
        .refreshable {
            await sync.refreshHomework(date)
        }
    }
}
