//
//  HomeworkView.swift
//  Libellule
//
//  Created by Jules on 09/09/2026.
//

import SwiftUI

struct HomeworkView: View {
    private let datesRange: ClosedRange<Date>
    private let dates: [Date]
    
    @State private var showingDatePicker = false
    @State private var temporarySelection: Date
    
    @State private var visibleDate = ScrollPosition(idType: Date.self)
    
    private var dateBinding: Binding<Date> {
        Binding<Date>(
            get: {
                visibleDate.viewID(type: Date.self) ?? temporarySelection
            },
            set: { newValue in
                visibleDate = ScrollPosition(id: newValue)
            }
        )
    }
    
    init(datesRange: ClosedRange<Date>) {
        var calendar = Calendar.current
        calendar.firstWeekday = 2
        
        let start = calendar.date(from: calendar.dateComponents(
            [.yearForWeekOfYear, .weekOfYear],
            from: datesRange.lowerBound
        )) ?? datesRange.lowerBound
        
        var end = calendar.date(from: calendar.dateComponents(
            [.yearForWeekOfYear, .weekOfYear],
            from: datesRange.upperBound
        )) ?? datesRange.upperBound
        
        end = calendar.date(byAdding: .day, value: 6, to: end) ?? end
        
        self.datesRange = start...end
            
        var dates = [start]
        dates.append(
            contentsOf: calendar.dates(byAdding: .weekOfYear, startingAt: start, in: .distantPast..<end)
        )
        self.dates = dates
        
        var today = min(max(Date.now, start), end)
        today = calendar.date(from: calendar.dateComponents(
            [.yearForWeekOfYear, .weekOfYear],
            from: today
        )) ?? today
        
        self.temporarySelection = today
        self._visibleDate = State(initialValue: .init(id: today))
    }
    
    var body: some View {
        NavigationView {
            ScrollView(.horizontal) {
                LazyHStack(spacing: 0) {
                    ForEach(dates, id: \.self) { date in
                        HomeworkWeek(date: date)
                            .containerRelativeFrame(.horizontal)
                            .id(date)
                    }
                }
                .scrollTargetLayout()
            }
            .scrollTargetBehavior(.paging)
            .scrollPosition($visibleDate)
            .scrollIndicators(.hidden)
            .navigationTitle("Semaine \(dateBinding.wrappedValue.formatted(.dateTime.week(.defaultDigits)))")
            .navigationSubtitle(formatWeek(monday: dateBinding.wrappedValue))
            .navigationBarTitleDisplayMode(.inline)
            .toolbarTitleMenu {
                Button("Choisir une semaine", systemImage: "calendar") {
                    temporarySelection = dateBinding.wrappedValue
                    showingDatePicker = true
                }
            }
            .sheet(isPresented: $showingDatePicker) {
                NavigationStack {
                    WeekOfYearCalendarView(selection: $temporarySelection, in: datesRange)
                        .navigationTitle("Date")
                        .navigationBarTitleDisplayMode(.inline)
                        .toolbar {
                            ToolbarItem(placement: .confirmationAction) {
                                Button(role: .confirm) {
                                    dateBinding.wrappedValue = temporarySelection
                                    showingDatePicker = false
                                }
                            }
                        }
                        .padding()
                }
                    .presentationDetents([.medium])
            }
        }
    }
    
    func formatWeek(monday: Date) -> String {
        let sunday = Calendar.current.date(byAdding: .day, value: 6, to: monday)!
        
        let weekFormatter = DateIntervalFormatter()
        weekFormatter.dateStyle = .long
        weekFormatter.timeStyle = .none
        
        return weekFormatter.string(from: DateInterval(start: monday, end: sunday))!
    }
}
