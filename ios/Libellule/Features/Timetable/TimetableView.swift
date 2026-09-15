//
//  TimetableView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct TimetableView: View {
    private let datesRange: ClosedRange<Date>
    private let dates: [String]

    @State private var showingDatePicker = false
    @State private var temporarySelection: Date
    
    @State private var visibleDate: ScrollPosition
    
    private var dateBinding: Binding<Date> {
        Binding<Date>(
            get: {
                DateFormatter.date.date(from: visibleDate.viewID as! String)!
            },
            set: { newValue in
                visibleDate = ScrollPosition(id: DateFormatter.date.string(from: newValue))
            }
        )
    }
    
    init(datesRange: ClosedRange<Date>) {
        self.datesRange = datesRange
        
        var dates: [String] = []
        let calendar = Calendar.current
        
        var currentDate = calendar.startOfDay(for: datesRange.lowerBound)
        let end = calendar.startOfDay(for: datesRange.upperBound)
        
        while currentDate <= end {
            dates.append(DateFormatter.date.string(from: currentDate))
            guard let nextDate = calendar.date(byAdding: .day, value: 1, to: currentDate) else { break }
            currentDate = nextDate
        }
        
        self.dates = dates
        
        let today = min(max(Date.now, datesRange.lowerBound), datesRange.upperBound)
        self.temporarySelection = today
        self._visibleDate = State(initialValue: .init(id: DateFormatter.date.string(from: today)))
    }
    
    var body: some View {
        NavigationStack {
            ScrollView(.horizontal) {
                LazyHStack(spacing: 0) {
                    ForEach(dates, id: \.self) { date in
                        DayView(date: date)
                            .containerRelativeFrame(.horizontal)
                    }
                }
                .scrollTargetLayout()
            }
            .scrollTargetBehavior(.paging)
            .scrollPosition($visibleDate)
            .scrollIndicators(.hidden)
            .navigationTitle(dateBinding.wrappedValue.formatted(.dateTime.weekday(.wide)).localizedCapitalized)
            .navigationSubtitle(dateBinding.wrappedValue.formatted(date: .long, time: .omitted))
            .navigationBarTitleDisplayMode(.inline)
            .toolbarTitleMenu {
                Button("Choisir une date", systemImage: "calendar") {
                    temporarySelection = dateBinding.wrappedValue
                    showingDatePicker = true
                }
            }
            .toolbar {
                NavigationLink {
                    MenuView(date: dateBinding.wrappedValue)
                } label: {
                    Image(systemName: "menucard")
                }

            }
            .sheet(isPresented: $showingDatePicker) {
                NavigationStack {
                    SingleDateCalendarView(selection: $temporarySelection, in: datesRange)
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
}
