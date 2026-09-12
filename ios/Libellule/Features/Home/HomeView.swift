//
//  HomeView.swift
//  Libellule
//
//  Created by Jules on 09/09/2026.
//

import SwiftUI
import SwiftData

struct HomeView: View {
    private let datesRange: ClosedRange<Date>
    
    @Binding var currentTab: AppTab
    
    @Query private var nextDay: [CachedDay]
    
    init(datesRange: ClosedRange<Date>, currentTab: Binding<AppTab>) {
        self.datesRange = datesRange
        
        let startOfToday = Calendar.current.startOfDay(for: .now)
        let today = DateFormatter.date.string(from: min(max(startOfToday, datesRange.lowerBound), datesRange.upperBound))
        
        _nextDay = Query(FetchDescriptor<CachedDay>(predicate: #Predicate { $0.date >= today && !$0.lessons.isEmpty }, sortBy: [SortDescriptor(\.date)]))
        
        _currentTab = currentTab
    }
    
    var body: some View {
        NavigationStack {
            VStack {
                if let nextDay = nextDay.first {
                    LessonList(lessons: Array(nextDay.lessons.prefix(3)))
                    Button("Voir tout l'emploi du temps") {
                        currentTab = .timetable
                    }
                }
            }
            .navigationTitle("Accueil")
        }
    }
}
