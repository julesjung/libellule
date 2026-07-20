//
//  GradesView.swift
//  Pronote
//
//  Created by Jules on 20/07/2026.
//

import SwiftUI
import PronoteKit

struct GradesView: View {
    var gradesData: GradesData
    
    var body: some View {
        List(gradesData.assignments, id: \.self) { assignment in
            HStack {
                VStack(alignment: .leading) {
                    Text(assignment.subject.name).lineLimit(1)
                    Text(assignment.label).font(.caption).lineLimit(1)
                }
                Spacer()
                Text("\(assignment.grade))/\(Int(assignment.scale))")
            }
        }
        .task {
            print(gradesData)
        }
        .listStyle(.plain)
        .navigationTitle(Text("Notes"))
    }
}

#Preview {
    GradesView(gradesData: GradesData(subjects: [], assignments: []))
}
