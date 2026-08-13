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
    
    @State private var showingDatePicker = false
    
    var body: some View {
        NavigationStack {
            VStack {
                dateHeader
                
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
                            LazyVStack(spacing: 0) {
                                ForEach(timetable.lessons, id: \.id) { lesson in
                                    LessonView(lesson: lesson)
                                }
                            }
                        }
                    }
                }
                
                Spacer()
            }
                .navigationTitle("Emploi du temps")
        }
    }
    
    private var dateHeader: some View {
        @Bindable var store = store
        
        return HStack {
            Button {
                store.previousDay()
            } label: {
                Image(systemName: "chevron.left")
            }
            .buttonStyle(.glass)
            .controlSize(.large)

            Button {
                showingDatePicker = true
            } label: {
                VStack(spacing: 2) {
                    Text(store.selectedDate.formatted(.dateTime.weekday(.wide)))
                        .font(.headline)
                    
                    Text(store.selectedDate.formatted(.dateTime.day().month(.wide)))
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                }
            }
            .sheet(isPresented: $showingDatePicker) {
                NavigationStack {
                    CalendarView(selection: $store.selectedDate, in: store.datesRange)
                        .padding(.horizontal)
                        .navigationTitle("Date")
                        .navigationBarTitleDisplayMode(.inline)
                        .toolbar {
                            ToolbarItem(placement: .confirmationAction) {
                                Button(role: .confirm) {
                                    showingDatePicker = false
                                }
                            }
                        }
                }
                .presentationDetents([.medium])
            }
            .buttonStyle(.glass)
            .buttonSizing(.flexible)
            
            Button {
                store.nextDay()
            } label: {
                Image(systemName: "chevron.right")
            }
            .buttonStyle(.glass)
            .controlSize(.large)
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
