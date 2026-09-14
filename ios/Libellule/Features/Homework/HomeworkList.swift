//
//  HomeworkList.swift
//  Libellule
//
//  Created by Jules on 13/09/2026.
//

import SwiftUI

struct HomeworkList: View {
    let groupedItems: [Date: [CachedHomeworkItem]]
    
    init(items: [CachedHomeworkItem]) {
        self.groupedItems = Dictionary(grouping: items, by: \.due)
    }
    
    var body: some View {
        List(groupedItems.sorted { $0.key < $1.key }, id: \.key) { (date, items) in
            Section(date.formatted(.dateTime.weekday(.wide).day(.defaultDigits).month(.wide))) {
                ForEach(items, id: \.id) { item in
                    HomeworkItem(item: item)
                }
            }
        }
    }
}
