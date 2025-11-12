#!/usr/bin/env python3
"""
Generate Japanese and Chinese translations for dialoguer i18n keys
"""
import json

# Translations for 207 new keys (ja/zh/zh-TW)
translations = {
    # Refactor command
    "refactor-goal-readability": {
        "ja": "コードの可読性・保守性向上",
        "zh": "提高代码可读性/可维护性",
        "zh-TW": "提高程式碼可讀性/可維護性"
    },
    "refactor-goal-dry": {
        "ja": "コード重複の削減（DRY原則）",
        "zh": "减少代码重复（DRY原则）",
        "zh-TW": "減少程式碼重複（DRY原則）"
    },
    "refactor-goal-performance": {
        "ja": "パフォーマンス改善",
        "zh": "改善性能",
        "zh-TW": "改善效能"
    },
    "refactor-goal-simplify": {
        "ja": "複雑なロジックの簡略化",
        "zh": "简化复杂逻辑",
        "zh-TW": "簡化複雜邏輯"
    },
    "refactor-goal-extract": {
        "ja": "再利用可能なコンポーネント・ユーティリティの抽出",
        "zh": "提取可重用组件/实用工具",
        "zh-TW": "提取可重用元件/實用工具"
    },
    "refactor-goal-type-safety": {
        "ja": "型安全性の向上",
        "zh": "改善类型安全",
        "zh-TW": "改善型別安全"
    },
    "refactor-goal-modern": {
        "ja": "モダンなパターン・ベストプラクティスへの更新",
        "zh": "更新为现代模式/最佳实践",
        "zh-TW": "更新為現代模式/最佳實踐"
    },
    "refactor-goal-debt": {
        "ja": "技術的負債の削減",
        "zh": "减少技术债务",
        "zh-TW": "減少技術債務"
    },
    "refactor-goal-prepare": {
        "ja": "新機能への準備",
        "zh": "为新功能做准备",
        "zh-TW": "為新功能做準備"
    },
    "refactor-goal-smells": {
        "ja": "コードスメルの修正",
        "zh": "修复代码异味",
        "zh-TW": "修復程式碼異味"
    },
    "refactor-goal-prompt": {
        "ja": "リファクタリング目標を選択（スペースで選択、Enterで確定）",
        "zh": "选择重构目标（空格选择，Enter确认）",
        "zh-TW": "選擇重構目標（空格選擇，Enter確認）"
    },

    "refactor-scope-single-function": {
        "ja": "単一関数・メソッド（影響小）",
        "zh": "单个函数/方法（影响小）",
        "zh-TW": "單一函式/方法（影響小）"
    },
    "refactor-scope-single-file": {
        "ja": "単一ファイル・モジュール（影響中）",
        "zh": "单个文件/模块（影响中）",
        "zh-TW": "單一檔案/模組（影響中）"
    },
    "refactor-scope-multiple-files": {
        "ja": "複数の関連ファイル（影響大）",
        "zh": "多个相关文件（影响大）",
        "zh-TW": "多個相關檔案（影響大）"
    },
    "refactor-scope-system-wide": {
        "ja": "横断的関心事（システム全体に影響）",
        "zh": "跨领域关注（系统范围影响）",
        "zh-TW": "跨領域關注（系統範圍影響）"
    },
    "refactor-scope-prompt": {
        "ja": "リファクタリング範囲",
        "zh": "重构范围",
        "zh-TW": "重構範圍"
    },

    "refactor-file-prompt": {
        "ja": "ファイル",
        "zh": "文件",
        "zh-TW": "檔案"
    },

    "refactor-test-question": {
        "ja": "対象コードに既存のテストはありますか？",
        "zh": "目标代码是否有现有测试？",
        "zh-TW": "目標程式碼是否有現有測試？"
    },
    "refactor-continue-no-tests": {
        "ja": "テストなしで続行しますか？（推奨しません）",
        "zh": "是否在没有测试的情况下继续？（不推荐）",
        "zh-TW": "是否在沒有測試的情況下繼續？（不推薦）"
    },

    "refactor-pattern-extract-function": {
        "ja": "関数・メソッドの抽出",
        "zh": "提取函数/方法",
        "zh-TW": "提取函式/方法"
    },
    "refactor-pattern-extract-component": {
        "ja": "コンポーネント・モジュールの抽出",
        "zh": "提取组件/模块",
        "zh-TW": "提取元件/模組"
    },
    "refactor-pattern-inline": {
        "ja": "関数・変数のインライン化",
        "zh": "内联函数/变量",
        "zh-TW": "內聯函式/變數"
    },
    "refactor-pattern-rename": {
        "ja": "リネーム（命名改善）",
        "zh": "重命名（改进命名）",
        "zh-TW": "重新命名（改進命名）"
    },
    "refactor-pattern-move": {
        "ja": "関数・クラスの移動",
        "zh": "移动函数/类",
        "zh-TW": "移動函式/類別"
    },
    "refactor-pattern-polymorphism": {
        "ja": "条件分岐をポリモーフィズムに置換",
        "zh": "用多态替换条件语句",
        "zh-TW": "用多型替換條件陳述式"
    },
    "refactor-pattern-parameter-object": {
        "ja": "パラメータオブジェクトの導入",
        "zh": "引入参数对象",
        "zh-TW": "引入參數物件"
    },
    "refactor-pattern-constants": {
        "ja": "マジックナンバーを定数に置換",
        "zh": "用常量替换魔法数字",
        "zh-TW": "用常數替換魔術數字"
    },
    "refactor-pattern-decompose": {
        "ja": "条件分岐の分解",
        "zh": "分解条件语句",
        "zh-TW": "分解條件陳述式"
    },
    "refactor-pattern-consolidate": {
        "ja": "重複コードの統合",
        "zh": "合并重复代码",
        "zh-TW": "合併重複程式碼"
    },
    "refactor-pattern-simplify": {
        "ja": "複雑な式の簡略化",
        "zh": "简化复杂表达式",
        "zh-TW": "簡化複雜表達式"
    },
    "refactor-pattern-guard": {
        "ja": "ネストした条件分岐をガード節に置換",
        "zh": "用保护子句替换嵌套条件",
        "zh-TW": "用保護子句替換巢狀條件"
    },
    "refactor-pattern-interface": {
        "ja": "インターフェース・型の抽出",
        "zh": "提取接口/类型",
        "zh-TW": "提取介面/型別"
    },
    "refactor-pattern-pipeline": {
        "ja": "ループをパイプライン（map/filter/reduce）に置換",
        "zh": "用管道（map/filter/reduce）替换循环",
        "zh-TW": "用管道（map/filter/reduce）替換迴圈"
    },
    "refactor-pattern-prompt": {
        "ja": "適用するリファクタリング技法を選択（スペースで選択、Enterで確定）",
        "zh": "选择要应用的重构技术（空格选择，Enter确认）",
        "zh-TW": "選擇要應用的重構技術（空格選擇，Enter確認）"
    },

    "refactor-step-prompt": {
        "ja": "ステップ {num}",
        "zh": "步骤 {num}",
        "zh-TW": "步驟 {num}"
    },

    "refactor-security-auth": {
        "ja": "認証・認可ロジックの変更",
        "zh": "认证/授权逻辑更改",
        "zh-TW": "認證/授權邏輯變更"
    },
    "refactor-security-validation": {
        "ja": "入力検証の変更",
        "zh": "输入验证更改",
        "zh-TW": "輸入驗證變更"
    },
    "refactor-security-sanitization": {
        "ja": "データサニタイズの変更",
        "zh": "数据清理更改",
        "zh-TW": "資料清理變更"
    },
    "refactor-security-access": {
        "ja": "アクセス制御の変更",
        "zh": "访问控制更改",
        "zh-TW": "存取控制變更"
    },
    "refactor-security-encryption": {
        "ja": "暗号化・復号化ロジックの変更",
        "zh": "加密/解密逻辑更改",
        "zh-TW": "加密/解密邏輯變更"
    },
    "refactor-security-api": {
        "ja": "APIエンドポイントの変更",
        "zh": "API端点更改",
        "zh-TW": "API端點變更"
    },
    "refactor-security-none": {
        "ja": "該当なし",
        "zh": "以上都不是",
        "zh-TW": "以上都不是"
    },
    "refactor-security-prompt": {
        "ja": "このリファクタリングはセキュリティ上重要な領域に影響しますか？（スペースで選択、Enterで確定）",
        "zh": "此重构是否影响这些安全敏感区域？（空格选择，Enter确认）",
        "zh-TW": "此重構是否影響這些安全敏感區域？（空格選擇，Enter確認）"
    },

    "refactor-status-planning": {
        "ja": "計画中（未着手）",
        "zh": "计划中（尚未开始）",
        "zh-TW": "計劃中（尚未開始）"
    },
    "refactor-status-progress": {
        "ja": "進行中（リファクタリング実施中）",
        "zh": "进行中（正在重构）",
        "zh-TW": "進行中（正在重構）"
    },
    "refactor-status-testing": {
        "ja": "テスト中（リファクタリング完了、検証中）",
        "zh": "测试中（重构完成，验证中）",
        "zh-TW": "測試中（重構完成，驗證中）"
    },
    "refactor-status-review": {
        "ja": "レビュー待ち（コードレビュー準備完了）",
        "zh": "审查中（准备代码审查）",
        "zh-TW": "審查中（準備程式碼審查）"
    },
    "refactor-status-completed": {
        "ja": "完了（マージ済み）",
        "zh": "已完成（已合并）",
        "zh-TW": "已完成（已合併）"
    },
    "refactor-status-prompt": {
        "ja": "現在の状態",
        "zh": "当前状态",
        "zh-TW": "目前狀態"
    },

    "refactor-measure-improvements": {
        "ja": "改善効果を測定しましたか？",
        "zh": "是否测量了改进效果？",
        "zh-TW": "是否測量了改進效果？"
    },
    "refactor-improvement-prompt": {
        "ja": "改善内容",
        "zh": "改进内容",
        "zh-TW": "改進內容"
    },

    # Feature command
    "feature-problem-prompt": {
        "ja": "❓ 問題・課題の説明",
        "zh": "❓ 问题说明",
        "zh-TW": "❓ 問題說明"
    },
    "feature-users-prompt": {
        "ja": "👥 対象ユーザー",
        "zh": "👥 目标用户",
        "zh-TW": "👥 目標使用者"
    },
    "feature-criterion-prompt": {
        "ja": "基準 {num}",
        "zh": "标准 {num}",
        "zh-TW": "標準 {num}"
    },

    "feature-type-ui": {
        "ja": "新しいUIコンポーネント・ページ",
        "zh": "新UI组件/页面",
        "zh-TW": "新UI元件/頁面"
    },
    "feature-type-api": {
        "ja": "新しいAPIエンドポイント・サービス",
        "zh": "新API端点/服务",
        "zh-TW": "新API端點/服務"
    },
    "feature-type-data": {
        "ja": "データモデルの変更",
        "zh": "数据模型更改",
        "zh-TW": "資料模型變更"
    },
    "feature-type-algorithm": {
        "ja": "アルゴリズム・ビジネスロジック",
        "zh": "算法/业务逻辑",
        "zh-TW": "演算法/業務邏輯"
    },
    "feature-type-integration": {
        "ja": "外部サービスとの統合",
        "zh": "与外部服务集成",
        "zh-TW": "與外部服務整合"
    },
    "feature-type-performance": {
        "ja": "パフォーマンス向上",
        "zh": "性能增强",
        "zh-TW": "效能增強"
    },
    "feature-type-tooling": {
        "ja": "開発ツール・インフラストラクチャ",
        "zh": "开发工具/基础设施",
        "zh-TW": "開發工具/基礎架構"
    },
    "feature-type-security": {
        "ja": "セキュリティ機能",
        "zh": "安全功能",
        "zh-TW": "安全功能"
    },
    "feature-type-accessibility": {
        "ja": "アクセシビリティ改善",
        "zh": "无障碍改进",
        "zh-TW": "無障礙改進"
    },
    "feature-type-other": {
        "ja": "その他",
        "zh": "其他",
        "zh-TW": "其他"
    },
    "feature-type-prompt": {
        "ja": "機能タイプ",
        "zh": "功能类型",
        "zh-TW": "功能類型"
    },

    "feature-complexity-small": {
        "ja": "小規模（< 1日、~1-2ファイル）",
        "zh": "小型（< 1天，~1-2个文件）",
        "zh-TW": "小型（< 1天，~1-2個檔案）"
    },
    "feature-complexity-medium": {
        "ja": "中規模（1-3日、~3-5ファイル）",
        "zh": "中型（1-3天，~3-5个文件）",
        "zh-TW": "中型（1-3天，~3-5個檔案）"
    },
    "feature-complexity-large": {
        "ja": "大規模（3-7日、~6-10ファイル）",
        "zh": "大型（3-7天，~6-10个文件）",
        "zh-TW": "大型（3-7天，~6-10個檔案）"
    },
    "feature-complexity-xlarge": {
        "ja": "超大規模（1-2週間、10+ファイル）",
        "zh": "特大型（1-2周，10+个文件）",
        "zh-TW": "特大型（1-2週，10+個檔案）"
    },
    "feature-complexity-prompt": {
        "ja": "⚖️  予想される複雑度",
        "zh": "⚖️  预计复杂度",
        "zh-TW": "⚖️  預計複雜度"
    },

    "feature-branch-create": {
        "ja": "新しい機能ブランチを作成しますか？",
        "zh": "是否创建新功能分支？",
        "zh-TW": "是否建立新功能分支？"
    },
    "feature-branch-name": {
        "ja": "ブランチ名",
        "zh": "分支名称",
        "zh-TW": "分支名稱"
    },

    "feature-design-ui": {
        "ja": "UI/UXデザイン（ワイヤーフレーム、モックアップ）",
        "zh": "UI/UX设计（线框图、模型）",
        "zh-TW": "UI/UX設計（線框圖、模型）"
    },
    "feature-design-schema": {
        "ja": "データベーススキーマの変更",
        "zh": "数据库架构更改",
        "zh-TW": "資料庫架構變更"
    },
    "feature-design-api": {
        "ja": "API契約・インターフェース設計",
        "zh": "API契约/接口设计",
        "zh-TW": "API契約/介面設計"
    },
    "feature-design-state": {
        "ja": "状態管理アプローチ",
        "zh": "状态管理方法",
        "zh-TW": "狀態管理方法"
    },
    "feature-design-architecture": {
        "ja": "コンポーネントアーキテクチャ",
        "zh": "组件架构",
        "zh-TW": "元件架構"
    },
    "feature-design-security": {
        "ja": "セキュリティ考慮事項",
        "zh": "安全考虑",
        "zh-TW": "安全考量"
    },
    "feature-design-performance": {
        "ja": "パフォーマンス考慮事項",
        "zh": "性能考虑",
        "zh-TW": "效能考量"
    },
    "feature-design-testing": {
        "ja": "テスト戦略",
        "zh": "测试策略",
        "zh-TW": "測試策略"
    },
    "feature-design-docs": {
        "ja": "ドキュメント要件",
        "zh": "文档要求",
        "zh-TW": "文件要求"
    },
    "feature-design-accessibility": {
        "ja": "アクセシビリティ要件",
        "zh": "无障碍要求",
        "zh-TW": "無障礙要求"
    },
    "feature-design-prompt": {
        "ja": "関連する設計考慮事項を選択（スペースで選択、Enterで確定）",
        "zh": "选择相关设计考虑（空格选择，Enter确认）",
        "zh-TW": "選擇相關設計考量（空格選擇，Enter確認）"
    },

    "feature-file-prompt": {
        "ja": "ファイル",
        "zh": "文件",
        "zh-TW": "檔案"
    },

    "feature-dependencies-question": {
        "ja": "この機能は新しい依存関係が必要ですか？",
        "zh": "此功能是否需要新的依赖项？",
        "zh-TW": "此功能是否需要新的相依性？"
    },
    "feature-dependency-prompt": {
        "ja": "依存関係",
        "zh": "依赖项",
        "zh-TW": "相依性"
    },

    "feature-test-unit": {
        "ja": "ユニットテスト（独立したコンポーネントテスト）",
        "zh": "单元测试（独立组件测试）",
        "zh-TW": "單元測試（獨立元件測試）"
    },
    "feature-test-integration": {
        "ja": "統合テスト（コンポーネント間の相互作用）",
        "zh": "集成测试（组件交互）",
        "zh-TW": "整合測試（元件互動）"
    },
    "feature-test-e2e": {
        "ja": "E2Eテスト（完全なユーザーフロー）",
        "zh": "E2E测试（完整用户流程）",
        "zh-TW": "E2E測試（完整使用者流程）"
    },
    "feature-test-visual": {
        "ja": "ビジュアルリグレッションテスト",
        "zh": "视觉回归测试",
        "zh-TW": "視覺迴歸測試"
    },
    "feature-test-performance": {
        "ja": "パフォーマンステスト",
        "zh": "性能测试",
        "zh-TW": "效能測試"
    },
    "feature-test-accessibility": {
        "ja": "アクセシビリティテスト",
        "zh": "无障碍测试",
        "zh-TW": "無障礙測試"
    },
    "feature-test-security": {
        "ja": "セキュリティテスト",
        "zh": "安全测试",
        "zh-TW": "安全測試"
    },
    "feature-test-manual": {
        "ja": "手動テストチェックリスト",
        "zh": "手动测试清单",
        "zh-TW": "手動測試清單"
    },
    "feature-test-prompt": {
        "ja": "必要なテストタイプを選択（スペースで選択、Enterで確定）",
        "zh": "选择所需测试类型（空格选择，Enter确认）",
        "zh-TW": "選擇所需測試類型（空格選擇，Enter確認）"
    },

    "feature-doc-readme": {
        "ja": "READMEの更新",
        "zh": "更新README",
        "zh-TW": "更新README"
    },
    "feature-doc-api": {
        "ja": "APIドキュメント",
        "zh": "API文档",
        "zh-TW": "API文件"
    },
    "feature-doc-comments": {
        "ja": "コードコメント・JSDoc",
        "zh": "代码注释/JSDoc",
        "zh-TW": "程式碼註解/JSDoc"
    },
    "feature-doc-guide": {
        "ja": "ユーザーガイド・チュートリアル",
        "zh": "用户指南/教程",
        "zh-TW": "使用者指南/教學"
    },
    "feature-doc-adr": {
        "ja": "アーキテクチャ決定記録（ADR）",
        "zh": "架构决策记录（ADR）",
        "zh-TW": "架構決策記錄（ADR）"
    },
    "feature-doc-migration": {
        "ja": "移行ガイド（破壊的変更の場合）",
        "zh": "迁移指南（如有破坏性更改）",
        "zh-TW": "遷移指南（如有破壞性變更）"
    },
    "feature-doc-prompt": {
        "ja": "作成・更新するドキュメントを選択（スペースで選択、Enterで確定）",
        "zh": "选择要创建/更新的文档（空格选择，Enter确认）",
        "zh-TW": "選擇要建立/更新的文件（空格選擇，Enter確認）"
    },

    "feature-status-planning": {
        "ja": "計画中（要件収集完了、開始準備完了）",
        "zh": "计划中（需求已收集，准备开始）",
        "zh-TW": "計劃中（需求已收集，準備開始）"
    },
    "feature-status-progress": {
        "ja": "進行中（実装中）",
        "zh": "进行中（正在实施）",
        "zh-TW": "進行中（正在實作）"
    },
    "feature-status-testing": {
        "ja": "テスト中（実装完了、テスト進行中）",
        "zh": "测试中（实施完成，测试进行中）",
        "zh-TW": "測試中（實作完成，測試進行中）"
    },
    "feature-status-review": {
        "ja": "レビュー待ち（コードレビュー準備完了）",
        "zh": "审查中（准备代码审查）",
        "zh-TW": "審查中（準備程式碼審查）"
    },
    "feature-status-completed": {
        "ja": "完了（マージ済み・デプロイ済み）",
        "zh": "已完成（已合并并部署）",
        "zh-TW": "已完成（已合併並部署）"
    },
    "feature-status-prompt": {
        "ja": "現在の状態",
        "zh": "当前状态",
        "zh-TW": "目前狀態"
    },

    "feature-learning-prompt": {
        "ja": "この機能の実装から何を学びましたか？",
        "zh": "从实施此功能中学到了什么？",
        "zh-TW": "從實作此功能中學到了什麼？"
    },

    # Optimize command
    "optimize-issue-page-load": {
        "ja": "ページ読み込み・レンダリングが遅い",
        "zh": "页面加载/渲染缓慢",
        "zh-TW": "頁面載入/渲染緩慢"
    },
    "optimize-issue-api": {
        "ja": "APIレスポンス時間",
        "zh": "API响应时间",
        "zh-TW": "API回應時間"
    },
    "optimize-issue-database": {
        "ja": "データベースクエリパフォーマンス",
        "zh": "数据库查询性能",
        "zh-TW": "資料庫查詢效能"
    },
    "optimize-issue-memory": {
        "ja": "メモリ使用量・リーク",
        "zh": "内存使用/泄漏",
        "zh-TW": "記憶體使用/洩漏"
    },
    "optimize-issue-cpu": {
        "ja": "CPU集約的な処理",
        "zh": "CPU密集型操作",
        "zh-TW": "CPU密集型操作"
    },
    "optimize-issue-network": {
        "ja": "ネットワークリクエスト（多すぎる、大きすぎる）",
        "zh": "网络请求（过多、过大）",
        "zh-TW": "網路請求（過多、過大）"
    },
    "optimize-issue-bundle": {
        "ja": "バンドルサイズ（JavaScript/CSS）",
        "zh": "包大小（JavaScript/CSS）",
        "zh-TW": "套件大小（JavaScript/CSS）"
    },
    "optimize-issue-image": {
        "ja": "画像・アセット読み込み",
        "zh": "图像/资源加载",
        "zh-TW": "圖像/資源載入"
    },
    "optimize-issue-animation": {
        "ja": "アニメーション・スクロールパフォーマンス",
        "zh": "动画/滚动性能",
        "zh-TW": "動畫/捲動效能"
    },
    "optimize-issue-search": {
        "ja": "検索・フィルタリング操作",
        "zh": "搜索/过滤操作",
        "zh-TW": "搜尋/篩選操作"
    },
    "optimize-issue-prompt": {
        "ja": "パフォーマンス問題を選択（スペースで選択、Enterで確定）",
        "zh": "选择性能问题（空格选择，Enter确认）",
        "zh-TW": "選擇效能問題（空格選擇，Enter確認）"
    },

    "optimize-baseline-question": {
        "ja": "現在の（ベースライン）パフォーマンスを測定しましたか？",
        "zh": "是否测量了当前（基准）性能？",
        "zh-TW": "是否測量了目前（基準）效能？"
    },
    "optimize-continue-no-baseline": {
        "ja": "ベースラインなしで続行しますか？（推奨しません）",
        "zh": "是否在没有基准的情况下继续？（不推荐）",
        "zh-TW": "是否在沒有基準的情況下繼續？（不推薦）"
    },
    "optimize-baseline-prompt": {
        "ja": "ベースライン測定値",
        "zh": "基准指标",
        "zh-TW": "基準指標"
    },

    "optimize-bottleneck-images": {
        "ja": "大きい・最適化されていない画像",
        "zh": "大型/未优化图像",
        "zh-TW": "大型/未最佳化圖像"
    },
    "optimize-bottleneck-javascript": {
        "ja": "過剰なJavaScript実行",
        "zh": "过度的JavaScript执行",
        "zh-TW": "過度的JavaScript執行"
    },
    "optimize-bottleneck-rerenders": {
        "ja": "不要な再レンダリング（React/Vue）",
        "zh": "不必要的重新渲染（React/Vue）",
        "zh-TW": "不必要的重新渲染（React/Vue）"
    },
    "optimize-bottleneck-blocking": {
        "ja": "ブロッキング・同期処理",
        "zh": "阻塞/同步操作",
        "zh-TW": "阻塞/同步操作"
    },
    "optimize-bottleneck-n-plus-one": {
        "ja": "N+1クエリ問題",
        "zh": "N+1查询问题",
        "zh-TW": "N+1查詢問題"
    },
    "optimize-bottleneck-indexes": {
        "ja": "データベースインデックスの欠如",
        "zh": "缺少数据库索引",
        "zh-TW": "缺少資料庫索引"
    },
    "optimize-bottleneck-data": {
        "ja": "大量のデータ転送",
        "zh": "大量数据传输",
        "zh-TW": "大量資料傳輸"
    },
    "optimize-bottleneck-algorithms": {
        "ja": "非効率なアルゴリズム（O(n²)以上）",
        "zh": "低效算法（O(n²)或更差）",
        "zh-TW": "低效演算法（O(n²)或更差）"
    },
    "optimize-bottleneck-memory-leaks": {
        "ja": "メモリリーク",
        "zh": "内存泄漏",
        "zh-TW": "記憶體洩漏"
    },
    "optimize-bottleneck-requests": {
        "ja": "ネットワークリクエストが多すぎる",
        "zh": "网络请求过多",
        "zh-TW": "網路請求過多"
    },
    "optimize-bottleneck-libraries": {
        "ja": "最適化されていないサードパーティライブラリ",
        "zh": "未优化的第三方库",
        "zh-TW": "未最佳化的第三方函式庫"
    },
    "optimize-bottleneck-css": {
        "ja": "CSSレイアウトスラッシング",
        "zh": "CSS布局抖动",
        "zh-TW": "CSS版面抖動"
    },
    "optimize-bottleneck-prompt": {
        "ja": "特定されたボトルネック（スペースで選択、Enterで確定）",
        "zh": "识别的瓶颈（空格选择，Enter确认）",
        "zh-TW": "識別的瓶頸（空格選擇，Enter確認）"
    },

    "optimize-technique-code-splitting": {
        "ja": "コード分割・遅延読み込み",
        "zh": "代码分割/延迟加载",
        "zh-TW": "程式碼分割/延遲載入"
    },
    "optimize-technique-images": {
        "ja": "画像最適化（圧縮、WebP、遅延読み込み）",
        "zh": "图像优化（压缩、WebP、延迟加载）",
        "zh-TW": "圖像最佳化（壓縮、WebP、延遲載入）"
    },
    "optimize-technique-memoization": {
        "ja": "メモ化（React.memo、useMemo、useCallback）",
        "zh": "记忆化（React.memo、useMemo、useCallback）",
        "zh-TW": "記憶化（React.memo、useMemo、useCallback）"
    },
    "optimize-technique-virtualization": {
        "ja": "仮想化（react-window、仮想スクロール）",
        "zh": "虚拟化（react-window、虚拟滚动）",
        "zh-TW": "虛擬化（react-window、虛擬捲動）"
    },
    "optimize-technique-debounce": {
        "ja": "デバウンス・スロットル（高コスト処理）",
        "zh": "防抖/节流（昂贵操作）",
        "zh-TW": "防抖/節流（昂貴操作）"
    },
    "optimize-technique-bundle-reduction": {
        "ja": "バンドルサイズ削減（ツリーシェイキング、未使用コード削除）",
        "zh": "减少包大小（摇树优化、删除未使用代码）",
        "zh-TW": "減少套件大小（搖樹最佳化、刪除未使用程式碼）"
    },
    "optimize-technique-css": {
        "ja": "CSS最適化（未使用削除、クリティカルCSS）",
        "zh": "优化CSS（删除未使用、关键CSS）",
        "zh-TW": "最佳化CSS（刪除未使用、關鍵CSS）"
    },
    "optimize-technique-web-workers": {
        "ja": "Web Workers（CPU作業のオフロード）",
        "zh": "Web Workers（分离CPU工作）",
        "zh-TW": "Web Workers（分離CPU工作）"
    },
    "optimize-technique-service-worker": {
        "ja": "Service Worker・キャッシング戦略",
        "zh": "Service Worker/缓存策略",
        "zh-TW": "Service Worker/快取策略"
    },
    "optimize-technique-query": {
        "ja": "データベースクエリ最適化（インデックス、クエリ書き換え）",
        "zh": "数据库查询优化（索引、查询重写）",
        "zh-TW": "資料庫查詢最佳化（索引、查詢重寫）"
    },
    "optimize-technique-caching": {
        "ja": "キャッシング（Redis、インメモリキャッシュ）",
        "zh": "缓存（Redis、内存缓存）",
        "zh-TW": "快取（Redis、記憶體快取）"
    },
    "optimize-technique-pooling": {
        "ja": "コネクションプーリング",
        "zh": "连接池",
        "zh-TW": "連線池"
    },
    "optimize-technique-async": {
        "ja": "async/awaitリファクタリング",
        "zh": "async/await重构",
        "zh-TW": "async/await重構"
    },
    "optimize-technique-batch": {
        "ja": "バッチ処理（往復削減）",
        "zh": "批处理（减少往返）",
        "zh-TW": "批次處理（減少往返）"
    },
    "optimize-technique-cdn": {
        "ja": "静的アセット用CDN",
        "zh": "静态资源CDN",
        "zh-TW": "靜態資源CDN"
    },
    "optimize-technique-algorithm": {
        "ja": "アルゴリズム改善（より良いデータ構造）",
        "zh": "算法改进（更好的数据结构）",
        "zh-TW": "演算法改進（更好的資料結構）"
    },
    "optimize-technique-parallel": {
        "ja": "並列処理",
        "zh": "并行处理",
        "zh-TW": "平行處理"
    },
    "optimize-technique-complexity": {
        "ja": "計算量削減",
        "zh": "降低计算复杂度",
        "zh-TW": "降低計算複雜度"
    },
    "optimize-technique-resource-reuse": {
        "ja": "リソースプーリング・再利用",
        "zh": "资源池/重用",
        "zh-TW": "資源池/重用"
    },
    "optimize-technique-prompt": {
        "ja": "適用する最適化技法を選択（スペースで選択、Enterで確定）",
        "zh": "选择要应用的优化技术（空格选择，Enter确认）",
        "zh-TW": "選擇要應用的最佳化技術（空格選擇，Enter確認）"
    },

    "optimize-step-prompt": {
        "ja": "ステップ {num}",
        "zh": "步骤 {num}",
        "zh-TW": "步驟 {num}"
    },
    "optimize-file-prompt": {
        "ja": "ファイル",
        "zh": "文件",
        "zh-TW": "檔案"
    },

    "optimize-target-prompt": {
        "ja": "パフォーマンス目標（例：「読み込み時間を1秒未満に」「50%高速化」）",
        "zh": "性能目标（例如：'将加载时间减少到<1秒'、'提高50%'）",
        "zh-TW": "效能目標（例如：「將載入時間減少到<1秒」、「提高50%」）"
    },

    "optimize-status-planning": {
        "ja": "計画中（分析完了、最適化準備完了）",
        "zh": "计划中（分析完成，准备优化）",
        "zh-TW": "計劃中（分析完成，準備最佳化）"
    },
    "optimize-status-progress": {
        "ja": "進行中（最適化実施中）",
        "zh": "进行中（正在实施优化）",
        "zh-TW": "進行中（正在實作最佳化）"
    },
    "optimize-status-measuring": {
        "ja": "測定中（最適化完了、メトリクス収集中）",
        "zh": "测量中（优化完成，收集指标）",
        "zh-TW": "測量中（最佳化完成，收集指標）"
    },
    "optimize-status-completed": {
        "ja": "完了（改善確認済み）",
        "zh": "已完成（已验证改进）",
        "zh-TW": "已完成（已驗證改進）"
    },
    "optimize-status-prompt": {
        "ja": "現在の状態",
        "zh": "当前状态",
        "zh-TW": "目前狀態"
    },

    "optimize-after-prompt": {
        "ja": "最適化後の測定値",
        "zh": "优化后指标",
        "zh-TW": "最佳化後指標"
    },
    "optimize-improvement-prompt": {
        "ja": "改善内容",
        "zh": "改进内容",
        "zh-TW": "改進內容"
    },

    # Research command
    "research-topic-prompt": {
        "ja": "📚 調査トピック",
        "zh": "📚 研究主题",
        "zh-TW": "📚 研究主題"
    },

    "research-motivation-learn": {
        "ja": "新しい技術・フレームワークの学習",
        "zh": "学习新技术/框架",
        "zh-TW": "學習新技術/框架"
    },
    "research-motivation-solve": {
        "ja": "特定の問題解決",
        "zh": "解决特定问题",
        "zh-TW": "解決特定問題"
    },
    "research-motivation-evaluate": {
        "ja": "代替案・選択肢の評価",
        "zh": "评估替代方案/选项",
        "zh-TW": "評估替代方案/選項"
    },
    "research-motivation-best-practices": {
        "ja": "ベストプラクティスの理解",
        "zh": "了解最佳实践",
        "zh-TW": "了解最佳實踐"
    },
    "research-motivation-performance": {
        "ja": "パフォーマンス最適化の調査",
        "zh": "性能优化研究",
        "zh-TW": "效能最佳化研究"
    },
    "research-motivation-architecture": {
        "ja": "アーキテクチャ・設計判断",
        "zh": "架构/设计决策",
        "zh-TW": "架構/設計決策"
    },
    "research-motivation-security": {
        "ja": "セキュリティ調査",
        "zh": "安全调查",
        "zh-TW": "安全調查"
    },
    "research-motivation-compatibility": {
        "ja": "互換性・統合調査",
        "zh": "兼容性/集成研究",
        "zh-TW": "相容性/整合研究"
    },
    "research-motivation-trends": {
        "ja": "業界トレンド・新興技術",
        "zh": "行业趋势/新兴技术",
        "zh-TW": "產業趨勢/新興技術"
    },
    "research-motivation-personal": {
        "ja": "個人スキル開発",
        "zh": "个人技能发展",
        "zh-TW": "個人技能發展"
    },
    "research-motivation-prompt": {
        "ja": "調査理由を選択（スペースで選択、Enterで確定）",
        "zh": "为什么进行此研究？（空格选择，Enter确认）",
        "zh-TW": "為什麼進行此研究？（空格選擇，Enter確認）"
    },

    "research-context-prompt": {
        "ja": "💡 背景（調査のきっかけは？）",
        "zh": "💡 背景（是什么引发了这项研究？）",
        "zh-TW": "💡 背景（是什麼引發了這項研究？）"
    },

    "research-question-prompt": {
        "ja": "質問 {num}",
        "zh": "问题 {num}",
        "zh-TW": "問題 {num}"
    },

    "research-scope-quick": {
        "ja": "簡易調査（< 1時間）",
        "zh": "快速调查（< 1小时）",
        "zh-TW": "快速調查（< 1小時）"
    },
    "research-scope-moderate": {
        "ja": "中規模調査（1-4時間）",
        "zh": "中等研究（1-4小时）",
        "zh-TW": "中等研究（1-4小時）"
    },
    "research-scope-deep": {
        "ja": "深掘り調査（1-2日）",
        "zh": "深入研究（1-2天）",
        "zh-TW": "深入研究（1-2天）"
    },
    "research-scope-extended": {
        "ja": "拡張調査（1週間以上）",
        "zh": "扩展研究（1周+）",
        "zh-TW": "擴展研究（1週+）"
    },
    "research-scope-prompt": {
        "ja": "調査時間",
        "zh": "时间投入",
        "zh-TW": "時間投入"
    },

    "research-activity-docs": {
        "ja": "ドキュメント・公式ガイドを読む",
        "zh": "阅读文档/官方指南",
        "zh-TW": "閱讀文件/官方指南"
    },
    "research-activity-blogs": {
        "ja": "ブログ記事・記事を読む",
        "zh": "阅读博客文章/文章",
        "zh-TW": "閱讀部落格文章/文章"
    },
    "research-activity-videos": {
        "ja": "動画・チュートリアルを見る",
        "zh": "观看视频/教程",
        "zh-TW": "觀看影片/教學"
    },
    "research-activity-source": {
        "ja": "ソースコード・サンプルを読む",
        "zh": "阅读源代码/示例",
        "zh-TW": "閱讀原始碼/範例"
    },
    "research-activity-experiment": {
        "ja": "ハンズオン実験",
        "zh": "实践实验",
        "zh-TW": "實作實驗"
    },
    "research-activity-poc": {
        "ja": "概念実証（PoC）構築",
        "zh": "构建概念验证",
        "zh-TW": "建立概念驗證"
    },
    "research-activity-benchmark": {
        "ja": "パフォーマンスベンチマーク",
        "zh": "性能基准测试",
        "zh-TW": "效能基準測試"
    },
    "research-activity-security": {
        "ja": "セキュリティ分析",
        "zh": "安全分析",
        "zh-TW": "安全分析"
    },
    "research-activity-community": {
        "ja": "コミュニティ調査（フォーラム、GitHubイシュー）",
        "zh": "社区研究（论坛、GitHub问题）",
        "zh-TW": "社群研究（論壇、GitHub問題）"
    },
    "research-activity-compare": {
        "ja": "代替案・競合製品の比較",
        "zh": "比较替代方案/竞争对手",
        "zh-TW": "比較替代方案/競爭對手"
    },
    "research-activity-papers": {
        "ja": "学術論文を読む",
        "zh": "阅读学术论文",
        "zh-TW": "閱讀學術論文"
    },
    "research-activity-experts": {
        "ja": "専門家・チームに相談",
        "zh": "咨询专家/团队",
        "zh-TW": "諮詢專家/團隊"
    },
    "research-activity-prompt": {
        "ja": "調査活動を選択（スペースで選択、Enterで確定）",
        "zh": "选择研究活动（空格选择，Enter确认）",
        "zh-TW": "選擇研究活動（空格選擇，Enter確認）"
    },

    "research-resource-prompt": {
        "ja": "リソース（URLまたは説明）",
        "zh": "资源（URL或描述）",
        "zh-TW": "資源（URL或描述）"
    },

    "research-experiment-question": {
        "ja": "ハンズオン実験を行いますか？",
        "zh": "是否进行实践实验？",
        "zh-TW": "是否進行實作實驗？"
    },
    "research-experiment-prompt": {
        "ja": "実験",
        "zh": "实验",
        "zh-TW": "實驗"
    },

    "research-status-planning": {
        "ja": "計画中（調査範囲定義中）",
        "zh": "计划中（定义研究范围）",
        "zh-TW": "計劃中（定義研究範圍）"
    },
    "research-status-progress": {
        "ja": "進行中（調査実施中）",
        "zh": "进行中（正在研究）",
        "zh-TW": "進行中（正在研究）"
    },
    "research-status-experimenting": {
        "ja": "実験中（ハンズオンテスト中）",
        "zh": "实验中（实践测试）",
        "zh-TW": "實驗中（實作測試）"
    },
    "research-status-analyzing": {
        "ja": "分析中（知見統合中）",
        "zh": "分析中（综合发现）",
        "zh-TW": "分析中（綜合發現）"
    },
    "research-status-completed": {
        "ja": "完了（調査完了）",
        "zh": "已完成（研究完成）",
        "zh-TW": "已完成（研究完成）"
    },
    "research-status-prompt": {
        "ja": "調査状態",
        "zh": "研究状态",
        "zh-TW": "研究狀態"
    },
}

# Load existing messages.json
with open('src/i18n/messages.json', 'r', encoding='utf-8') as f:
    messages = json.load(f)

# Add translations
for key, trans in translations.items():
    for lang in ['ja', 'zh', 'zh-TW']:
        messages[lang][key] = trans[lang]

# Sort keys alphabetically
for lang in messages:
    messages[lang] = dict(sorted(messages[lang].items()))

# Save back to messages.json
with open('src/i18n/messages.json', 'w', encoding='utf-8') as f:
    json.dump(messages, f, ensure_ascii=False, indent=2)

print(f"✅ Added translations for 207 keys")
print(f"New totals:")
for lang in ['en', 'ja', 'zh', 'zh-TW']:
    print(f"  {lang:6s}: {len(messages[lang])} keys")
