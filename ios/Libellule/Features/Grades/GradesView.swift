//
//  GradesView.swift
//  Libellule
//
//  Created by Jules on 20/07/2026.
//

import SwiftUI
import LibelluleKit

struct GradesView: View {
    @Environment(SessionStore.self) private var session
    @State var store: GradesStore

    var body: some View {
        NavigationStack {
            LoadableView(state: store.grades) {
                Task {
                    await store.loadGrades()
                }
            } content: { gradesData in
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
            .task {
                await store.loadPeriods()
            }
        }
    }
}
