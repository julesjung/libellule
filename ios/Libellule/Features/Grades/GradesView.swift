//
//  GradesView.swift
//  Libellule
//
//  Created by Jules on 20/07/2026.
//

import SwiftUI
import LibelluleKit

struct GradesView: View {
    @State var store: GradesStore
    
    var body: some View {
        NavigationStack {
            LoadableView(state: store.grades, retry: { await store.loadGrades() }) { gradesData in
                if gradesData.assignments.isEmpty {
                    ContentUnavailableView("Aucune note", systemImage: "text.page.slash.fill", description: Text("Aucune note pour cette période"))
                } else {
                    List(gradesData.assignments, id: \.self) { assignment in
                        HStack {
                            VStack(alignment: .leading) {
                                Text(assignment.subject.name).lineLimit(1)
                                Text(assignment.label).font(.caption).lineLimit(1)
                            }
                            Spacer()
                            switch assignment.grade {
                            case .graded(let grade):
                                Text("\(grade)/\(assignment.scale)")
                            case .absent:
                                Text("Absent")
                            default:
                                Text("Non Noté")
                            }
                        }
                    }
                    .listStyle(.plain)
                }
            }
            .task {
                await store.loadGrades()
            }
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Picker("Période", selection: $store.selectedPeriod) {
                        ForEach(store.periods, id: \.id) { period in
                            Text(period.name).tag(period)
                        }
                    }
                }
            }
        }
    }
}
