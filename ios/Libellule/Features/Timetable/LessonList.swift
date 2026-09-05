//
//  LessonList.swift
//  Libellule
//
//  Created by Jules on 31/08/2026.
//

import SwiftUI

struct LessonList: View {
    let lessons: [CachedLesson]
    
    var body: some View {
        if lessons.isEmpty {
            ContentUnavailableView("Aucun cours", systemImage: "beach.umbrella")
        } else {
            ScrollView {
                ForEach(lessons, id: \.id) { lesson in
                    LessonView(lesson: lesson)
                }
            }
            .scenePadding(.horizontal)
        }
    }
}
