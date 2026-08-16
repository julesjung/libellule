//
//  TimetableStore.swift
//  Libellule
//
//  Created by Jules on 12/08/2026.
//

import Foundation
import LibelluleKit

@Observable
final class TimetableStore {
    private let client: Client
    
    var datesRange: ClosedRange<Date>
    var selectedDate: Date {
        didSet { if oldValue != selectedDate { Task { await loadTimetable() } } }
    }
    var timetable: Loadable<Timetable> = .idle
    var menu: Loadable<Menu> = .idle
    
    init(client: Client) {
        self.client = client
        
        let boundaryDates = client.boundaryDates()
        
        let lowerBound = DateFormatter.date.date(from: boundaryDates.start)!
        let upperBound = DateFormatter.date.date(from: boundaryDates.end)!
        
        self.datesRange = lowerBound...upperBound
        self.selectedDate = min(max(Date.now, lowerBound), upperBound)
    }
    
    func clampToDatesRange(date: Date) -> Date {
        return min(max(date, datesRange.lowerBound), datesRange.upperBound)
    }
    
    func previousDay() {
        selectedDate = clampToDatesRange(
            date: Calendar.current.date(
                byAdding: .day,
                value: -1,
                to: selectedDate
            )!
        )
    }
    
    func nextDay() {
        selectedDate = clampToDatesRange(
            date: Calendar.current.date(
                byAdding: .day,
                value: 1,
                to: selectedDate
            )!
        )
    }

    func loadTimetable() async {
        guard datesRange.contains(selectedDate) else { return }
        
        self.timetable = .loading
        do {
            let lessons = try await client.timetable(date: DateFormatter.date.string(from: selectedDate))
            self.timetable = .loaded(lessons)
        } catch {
            self.timetable = .failed(error)
        }
    }
    
    func loadMenu() async {
        guard datesRange.contains(selectedDate) else { return }

        self.menu = .loading
        do {
            let date = DateFormatter.date.string(from: selectedDate)
            let menu = try await client.menu(date: date)
            self.menu = .loaded(menu)
        } catch {
            self.menu = .failed(error)
        }
    }
}
