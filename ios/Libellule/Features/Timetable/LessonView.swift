//
//  LessonView.swift
//  Libellule
//
//  Created by Jules on 13/08/2026.
//

import SwiftUI
import LibelluleKit

struct LessonView: View {
    @Environment(\.colorScheme) var colorScheme
    let lesson: CachedLesson

    var body: some View {
        let start = FFIDate.time.date(from: lesson.start)!
        let end = FFIDate.time.date(from: lesson.end)!

        let teachers = lesson.teachers.joined(separator: ", ")
        let rooms = lesson.rooms.joined(separator: ", ")

//        NavigationLink {
//            LessonDetails(lesson: lesson)
//        } label: {
            HStack(spacing: 12) {
                RoundedRectangle(cornerRadius: 10)
                    .foregroundStyle(Color(hex: lesson.color))
                    .frame(width: 10)

                VStack(alignment: .leading) {
                    Text(start.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                        .bold()

                    Spacer()

                    Text(end.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                        .bold()
                }
                .frame(minWidth: 50)

                VStack(alignment: .leading) {
                    Text(lesson.subjectName.localizedCapitalized)
                        .font(.headline)
                        .strikethrough(lesson.cancelled)
                        .foregroundStyle(lesson.cancelled ? .red : .primary)

                    if !teachers.isEmpty {
                        Text(teachers)
                            .font(.subheadline)
                    }
                    if !rooms.isEmpty {
                        Text(rooms)
                            .font(.subheadline)
                    }
                }
                Spacer()
            }
//        }
    }
}
