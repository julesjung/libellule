//
//  LessonDetails.swift
//  Libellule
//
//  Created by Jules on 09/09/2026.
//

import SwiftUI

struct LessonDetails: View {
    let lesson: CachedLesson
    
    var body: some View {
        Text("Hello, World!")
            .navigationTitle("mercredi 9 septembre")
            .navigationSubtitle("Updated just now")
            .navigationBarTitleDisplayMode(.inline)
    }
}
