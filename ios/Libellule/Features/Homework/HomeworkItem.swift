//
//  HomeworkItem.swift
//  Libellule
//
//  Created by Jules on 13/09/2026.
//

import SwiftUI

struct HomeworkItem: View {
    let item: CachedHomeworkItem
    
    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(item.subjectName)
                    .font(.headline)
                
                Text(item.contents.trimmingCharacters(in: .whitespacesAndNewlines))
                    .font(.subheadline)
            }
            Spacer()
            Button(action: { }) {
                Image(systemName: item.done ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(item.done ? .accent : .gray)
            }
            .disabled(true)
        }
    }
}
