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
            .toolbar {
                ToolbarItem {
                    Button {
                        store.previousDay()
                    } label: {
                        Image(systemName: "chevron.left")
                            .font(.headline)
                    }
                    .disabled(store.selectedDate == store.datesRange.lowerBound)
                }
                
                ToolbarItem {
                    DateSelector(selection: $store.selectedDate, in: store.datesRange)
                        .frame(maxWidth: .infinity)
                        .padding()
                }
                
                ToolbarItem {
                    Button {
                        store.nextDay()
                    } label: {
                        Image(systemName: "chevron.right")
                            .font(.headline)
                    }
                    .disabled(store.selectedDate == store.datesRange.upperBound)
                }
            }
        }
    }
}
