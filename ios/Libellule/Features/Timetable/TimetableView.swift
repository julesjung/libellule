//
//  TimetableView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct TimetableView: View {
    var store: TimetableStore
    
    var body: some View {
        @Bindable var store = store
        
        VStack {
            GlassDatePicker(selection: $store.selectedDate, in: store.datesRange)
            Spacer()
            LoadableView(state: store.timetable, retry: { await store.loadTimetable() }) { timetable in
                if timetable.lessons.isEmpty {
                    ContentUnavailableView(
                        "Aucun cours",
                        systemImage: "beach.umbrella",
                        description: Text("Profitez-en pour bien vous reposer !")
                    )
                } else {
                    ScrollView {
                        ForEach(timetable.lessons, id: \.id) { lesson in
                            LessonView(lesson: lesson)
                        }
                    }
                }
            }
            Spacer()
        }
    }
}

extension Color {
    init(hex: String) {
        let int = UInt64(hex.dropFirst(), radix: 16)!

        self.init(
            .sRGB,
            red: Double(int >> 16) / 255,
            green: Double(int >> 8 & 0xFF) / 255,
            blue:  Double(int & 0xFF) / 255,
            opacity: 1.0
        )
    }
}
