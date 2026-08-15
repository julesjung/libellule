//
//  TimetableView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct TimetableView: View {
    @State var store: TimetableStore

    var body: some View {
        NavigationStack {
            LoadableView(state: store.timetable, retry: { await store.loadTimetable() }) { timetable in
                GeometryReader { geometry in
                    DayPager(previous: store.previousDay, next: store.nextDay) {
                        let width = geometry.size.width
                        
                        HStack(spacing: 0) {
                            Color.clear
                                .frame(width: width)
                            
                            dayView(timetable: timetable)
                                .frame(width: width)
                            
                            if store.selectedDate != store.datesRange.upperBound {
                                Color.clear
                                    .frame(width: width)
                            }
                        }
                    }
                }
                .ignoresSafeArea()
            }
            .task {
                await store.loadTimetable()
            }
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    DateSelector(selection: $store.selectedDate, in: store.datesRange)
                        .frame(maxWidth: .infinity)
                        .padding()
                }
                
                ToolbarItem(placement: .topBarTrailing) {
                    NavigationLink {
                        MenuView(store: store)
                    } label: {
                        Image(systemName: "menucard")
                    }
                }
            }
        }
    }
    
    @ViewBuilder
    private func dayView(timetable: Timetable) -> some View {
        if timetable.lessons.isEmpty {
            ContentUnavailableView(
                "Aucun cours",
                systemImage: "beach.umbrella",
                description: Text("Profitez-en pour bien vous reposer !")
            )
        } else {
            ScrollView {
                LazyVStack(spacing: 0) {
                    ForEach(timetable.lessons, id: \.id) { lesson in
                        LessonView(lesson: lesson)
                    }
                }
            }
        }
    }
}
