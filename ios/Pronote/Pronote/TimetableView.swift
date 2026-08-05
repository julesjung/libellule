//
//  TimetableView.swift
//  Pronote
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import PronoteKit

struct TimetableView: View {
    var client: Client
    let formatter: DateFormatter
    @State private var selectedDate: Date
    @State private var timetable: Timetable?
    
    init(client: Client) {
        self.client = client
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        self.formatter = formatter
        self.selectedDate = formatter.date(from: "2026-04-01")!
    }
    
    var body: some View {
        VStack {
            DatePicker("Date", selection: $selectedDate, displayedComponents: [.date])
                .datePickerStyle(.compact)
                .onChange(of: selectedDate) {
                    timetable = nil
                }
            if timetable == nil {
                ProgressView("Chargement des cours")
                    .task {
                        timetable = try! await client.timetable(date: formatter.string(from: selectedDate))
                    }
            } else {
                List(timetable!.lessons, id: \.id) { lesson in
                    VStack(alignment: .leading) {
                        Text(lesson.start)
                            .font(.caption)
                        Text(lesson.subject.name)
                        HStack {
                            Image(systemName: "person")
                            Text(lesson.teachers.joined(separator: ", "))
                        }
                        HStack {
                            Image(systemName: "mappin")
                            Text(lesson.locations.map({ $0.name }).joined(separator: ", "))
                        }
                    }
                }
                .listStyle(.plain)
            }
        }
    }
}
